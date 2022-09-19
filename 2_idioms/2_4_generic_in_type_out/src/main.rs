use std::{ net::{ IpAddr, SocketAddr }, borrow::Cow };

fn main()
{
  println!( "Refactor me!" );

  let mut err = Error::new( "NO_USER" );
  err.status( 404 ).message( "User not found" );
}

#[ derive( Debug ) ]
pub struct Error
{
  code : Cow< 'static, str >,
  status : u16,
  message : Cow< 'static, str >,
}

impl Default for Error
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      code : "UNKNOWN".into(),
      status : 500,
      message : "Unknown error has happened.".into(),
    }
  }
}

impl Error
{
  pub fn new< S >( code : S ) -> Self
  where
    S : Into< Cow< 'static, str > >,
  {
    Self { code: code.into(), ..Self::default() }
  }

  pub fn status( &mut self, s : u16 ) -> &mut Self
  {
    self.status = s;
    self
  }

  pub fn message< S >( &mut self, m : S ) -> &mut Self
  where
    S : Into< Cow< 'static, str > >,
  {
    self.message = m.into();
    self
  }
}

#[ derive( Debug, Default ) ]
pub struct Server( Option< SocketAddr > );

impl Server
{
  pub fn bind< I >( &mut self, ip : I, port : u16 )
  where
    I : Into< IpAddr >
  {
    self.0 = Some( SocketAddr::new( ip.into(), port ) )
  }
}

#[ cfg( test ) ]
mod server_spec
{
  use super::*;

  mod bind
  {
    use std::net::{ Ipv4Addr, Ipv6Addr };

    use super::*;

    #[ test ]
    fn sets_provided_address_to_server()
    {
      let mut server = Server::default();

      server.bind( Ipv4Addr::new( 127, 0, 0, 1 ), 8080 );
      assert_eq!( format!( "{}", server.0.unwrap() ), "127.0.0.1:8080" );

      server.bind( "::1".parse::< Ipv6Addr >().unwrap(), 9911 );
      assert_eq!( format!( "{}", server.0.unwrap() ), "[::1]:9911" );
    }
  }
}
