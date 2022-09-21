use clap::Parser;

mod settings;

/// Prints its configuration to STDOUT.
#[ derive( Debug, Parser ) ]
#[ clap( author, version, about, long_about = None ) ]
struct Args
{
  /// Enables debug mode
  #[ clap( short, long, parse( from_flag ) ) ]
  debug : bool,

  /// Path to configuration file
  #[ clap( short, long, value_parser, default_value = "config.toml", env="CONF_FILE" ) ]
  conf : String,
}

fn main()
{
  let args = Args::parse();
  let settings = settings::Settings::try_new( args.conf, "CONF_" ).unwrap();
  dbg!( args.debug );
}
