use std::borrow::Cow;

const DEFAULT_PATH : &'static str = "/etc/app/app.conf";
const VAR_CONFIG_KEY : &'static str = "APP_CONF";
const CLI_CONFIG_ARG : &'static str = "--conf";

fn conf_path() -> Result< Cow< 'static, str >, String >
{
  match
  ( 
    std::env::args()
    .skip_while( | arg | arg != CLI_CONFIG_ARG )
    .skip( 1 ).next(),
    std::env::var( VAR_CONFIG_KEY ).ok()
  )
  {
    ( Some( path ), None ) => Ok( path.into() ),
    ( _, Some( path ) ) if !path.is_empty() => Ok ( path.into() ),
    _ => Ok( DEFAULT_PATH.into() )
  }
}

fn main()
{
  println!( "{:?}", conf_path() );
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn default()
  {
    assert_eq!( "/etc/app/app.conf", conf_path().unwrap() );
  }
  
  #[ test ]
  fn app_conf_is_empty()
  {
    std::env::set_var( VAR_CONFIG_KEY, "");
    assert_eq!( "/etc/app/app.conf", conf_path().unwrap() );
  }

  #[ test ]
  fn app_conf_is_not_empty()
  {
    std::env::set_var( VAR_CONFIG_KEY, "/home/user/app.conf");
    assert_eq!( "/home/user/app.conf", conf_path().unwrap() );
  }
}
