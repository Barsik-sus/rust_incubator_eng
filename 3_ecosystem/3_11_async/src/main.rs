use std::{ sync::Arc, io::BufRead };
use std::io::BufReader;
use clap::Parser;
use indicatif::{ ProgressBar, ProgressStyle };
use tokio::{ runtime::Builder, sync::Mutex };

mod file_loader;
use file_loader::LoadFile;


#[ derive( Parser ) ]
struct Args
{
  #[ clap( value_parser ) ]
  path : String,

  #[ clap( long, value_parser ) ]
  max_threads : Option< usize >,
}

fn main()
{
  let args = Args::parse();
  let client = reqwest::Client::new();
  let rt = Builder::new_multi_thread()
  .worker_threads( args.max_threads.unwrap_or( num_cpus::get() ) )
  .enable_all()
  .build().unwrap();

  rt.block_on( async move
  {
    let mut handlers = vec![];
    let files = BufReader::new( std::fs::File::open( args.path ).unwrap() )
    .lines()
    .map( | link |
    {
      let link = link.unwrap();
      let mut file_name = link.clone();
      file_name.retain( | c | !r#"/\*,".;:'"#.contains( c ) );
      LoadFile::new( client.clone(), link, file_name ).unwrap()
    })
    .collect::< Vec< _ > >();

    let pb = ProgressBar::new( files.len() as u64 );
    pb.set_style
    (
      ProgressStyle::with_template( "{msg} [{bar:40.cyan/blue}] {pos:>7}/{len:7}" ).unwrap()
      .progress_chars( "##>-" )
    );
    let pb = Arc::new( Mutex::new( pb ) );
    pb.lock().await.set_message( "Loading..." );
    for file in files
    {
      let pbc = Arc::clone( &pb );
      handlers.push( tokio::spawn( async move
      {
        file
        .load().await.unwrap();
        let pb = pbc.lock().await;
        pb.inc( 1 );
      }));
    }
    for handler in handlers { handler.await.unwrap() }
  });
}
