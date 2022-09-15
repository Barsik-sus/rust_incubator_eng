use std::{pin::Pin, rc::Rc};

trait MutMeSomehow 
{
  fn mut_me_somehow( self : Pin< &mut Self > );
}

trait SayHi: std::fmt::Debug 
{
  fn say_hi( self : Pin< &Self > ) 
  {
    println!( "Hi from {:?}", self )
  }
}

impl< T : Default > MutMeSomehow for Box< T >
{
  fn mut_me_somehow( mut self : Pin< &mut Self > ) 
  {
    self.set( Box::new( T::default() ) );
  }
}

impl< T : Default > MutMeSomehow for Rc< T >
{
  fn mut_me_somehow( mut self : Pin< &mut Self > ) 
  {
    self.set( Rc::new( T::default() ) );
  }
}

impl< T : Default > MutMeSomehow for Vec< T >
{
  fn mut_me_somehow( mut self : Pin< &mut Self > ) 
  {
    self.set( vec![ T::default() ] );
  }
}

impl MutMeSomehow for String
{
  fn mut_me_somehow( mut self : Pin< &mut Self > ) 
  {
    self.set( String::from( "Hello" ) );
  }
}

impl MutMeSomehow for &[ u8 ]
{
  fn mut_me_somehow( mut self : Pin< &mut Self > ) 
  {
    self.set( "Hello".as_bytes() );
  }
}

impl< T : std::fmt::Debug > SayHi for T {}


fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn mut_pinned_box()
  {
    let mut data = Box::new( "Hi" );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!(  "", *data );
  }

  #[ test ]
  fn mut_pinned_rc()
  {
    let mut data = Rc::new( 5 );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( 0, *data );
  }

  #[ test ]
  fn mut_pinned_vec()
  {
    let mut data = vec![ 1, 2, 3 ];
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( vec![ 0 ], data );
  }

  #[ test ]
  fn mut_pinned_string()
  {
    let mut data = String::from( "World" );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( "Hello", &data );
  }

  #[ test ]
  fn mut_pinned_slice_u8()
  {
    let mut data = String::from( "World" );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( "Hello", &data );
  }

  #[ test ]
  fn say_hi_box()
  {
    let data = Box::new( "Hi" );
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_rc()
  {
    let data = Rc::new( "Hi" );
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_vec()
  {
    let data = vec![ 1, 2, 3 ];
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_string()
  {
    let data = String::new();
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_slice_u8()
  {
    let data = &[ 1, 2, 3 ];
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_t()
  {
    let data = "Hi";
    Pin::new( &data ).say_hi();
  }
}
