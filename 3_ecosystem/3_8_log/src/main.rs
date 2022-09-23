use chrono::{ DateTime, Utc };
use serde::Serialize;
use std::{ sync::Mutex, io::{Write, BufRead} };
use log::{ info, trace, warn, SetLoggerError, LevelFilter, Level };

const APP_LOG_FILE : &'static str = "app.log";
const LOCAL_LOG_FILE : &'static str = "access.log";


#[ derive( Debug, Serialize ) ]
struct Log
{
  lvl : Level,
  file : &'static str,
  time : DateTime< Utc >,
  msg : String,
}

impl Log
{
  fn to_json( &self ) -> String
  {
    serde_json::to_string( self ).unwrap()
  }
}

pub struct GlobalLogger( Mutex< std::fs::File > );

impl GlobalLogger
{
  pub fn init() -> std::io::Result< Result< (), SetLoggerError > >
  {
    Ok
    (
      log::set_boxed_logger( Box::new(
      Self( Mutex::new
      (
        std::fs::File::options()
        .create( true ).append( true ).write( true )
        .open( &APP_LOG_FILE )?
      ))))
      .map( | () | log::set_max_level( LevelFilter::Trace ) )
    )
  }
}

impl log::Log for GlobalLogger
{
  fn enabled( &self, _metadata : &log::Metadata ) -> bool
  {
    true
  }

  fn log( &self, record : &log::Record )
  {
    let log = Log
    {
      lvl : record.level(),
      file : APP_LOG_FILE,
      time : Utc::now(),
      msg : record.args().to_string(),
    };
    if record.metadata().level() < Level::Warn
    {
      print!( "{}", log.to_json() );
    }
    else
    {
      eprintln!( "{}", log.to_json() );
    }
    self.0.lock().unwrap()
    .write_all
    (
      format!( "{}\n", log.to_json() ).as_bytes()
    ).unwrap()
  }

  fn flush( &self )
  {
    self.0.lock().unwrap().flush().unwrap()
  }
}

struct LocalLogger( Mutex< std::fs::File > );

impl LocalLogger
{
  pub fn init() -> std::io::Result< Result< (), SetLoggerError > >
  {
    Ok
    (
      log::set_boxed_logger( Box::new(
      Self( Mutex::new
      (
        std::fs::File::options()
        .create( true ).append( true ).write( true )
        .open( &LOCAL_LOG_FILE )?
      ))))
      .map( | () | log::set_max_level( LevelFilter::Trace ) )
    )
  }
}

impl log::Log for LocalLogger
{
  fn enabled( &self, _metadata : &log::Metadata ) -> bool
  {
    true
  }

  fn log( &self, record : &log::Record )
  {
    let log = Log
    {
      lvl : record.level(),
      file : LOCAL_LOG_FILE,
      time : Utc::now(),
      msg : record.args().to_string(),
    };
    self.0.lock().unwrap()
    .write_all
    (
      format!( "{}\n", log.to_json() ).as_bytes()
    ).unwrap()
  }

  fn flush( &self )
  {
    self.0.lock().unwrap().flush().unwrap()
  }
}

fn main()
{
  let _ = std::fs::remove_file( APP_LOG_FILE ).ok();

  GlobalLogger::init().unwrap().unwrap();
  info!( "Hi, I'm info" );
  warn!( "Hi, I'm warn look at me" );
  trace!( "Hi, Ama tracer" );

  let file = std::fs::File::open( APP_LOG_FILE ).unwrap();
  let reader = std::io::BufReader::new( file );
  let mut lines = reader.lines().map( | l | l.unwrap() );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"INFO\",\"file\":\"app.log\"") );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"WARN\",\"file\":\"app.log\"") );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"TRACE\",\"file\":\"app.log\"") );
}

#[ test ]
fn local_loger_test()
{
  let _ = std::fs::remove_file( LOCAL_LOG_FILE ).ok();

  LocalLogger::init().unwrap().unwrap();
  info!( "Hi, I'm local info" );
  warn!( "Hi, I'm local warn look at me" );
  trace!( "Hi, Ama tracer but local" );

  let file = std::fs::File::open( LOCAL_LOG_FILE ).unwrap();
  let reader = std::io::BufReader::new( file );
  let mut lines = reader.lines().map( | l | l.unwrap() );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"INFO\",\"file\":\"access.log\"") );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"WARN\",\"file\":\"access.log\"") );
  let line = lines.next().unwrap();
  assert!( line.contains( "\"lvl\":\"TRACE\",\"file\":\"access.log\"") );
}
