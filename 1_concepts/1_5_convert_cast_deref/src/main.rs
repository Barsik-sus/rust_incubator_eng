use rand::prelude::*;
use std::{ ops::Deref, borrow::Cow, cell::RefCell };


#[ derive( Debug, PartialEq ) ]
pub struct InvalidEmail;

#[ derive( Debug, PartialEq ) ]
pub struct EmailString< 'a >( Cow< 'a, str > );

impl< 'a > EmailString< 'a >
{
  pub fn try_new< S >( value : S ) -> Result< Self, InvalidEmail >
  where S : Into< Cow< 'a, str > > + Clone
  {
    if validator::validate_email( value.clone() )
    {
      Ok( Self( value.into() ) )
    }
    else 
    {
      Err( InvalidEmail )
    }
  }
}

#[ derive( Debug ) ]
pub struct Random< T >
{
  pub values : [ T; 3 ],
  _rng : RefCell< Box< ThreadRng > >
} 

impl< T > Random< T >
{
  pub fn new( first : T, second : T, third : T ) -> Self
  {
    Self 
    {
      values : [ first, second, third ],
      _rng : RefCell::new( Box::new( thread_rng() ) )
    }
  }
}

impl< T > Deref for Random< T >
{
  type Target = T;

  fn deref( &self ) -> &Self::Target 
  {
    &self.values.choose
    (
      &mut self._rng.borrow_mut().as_mut()
    ).unwrap()
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn valid_email()
  {
    assert_eq!
    ( 
      Ok( EmailString( "a@gmail.com".into() ) ),
      EmailString::try_new( "a@gmail.com" )
    );

    assert_eq!
    ( 
      Ok( EmailString( "some.email_address5@gmail.com".into() ) ),
      EmailString::try_new( "some.email_address5@gmail.com" )
    );

    let email = String::from( "a@gmail.com" );
    assert_eq!
    (
      Ok( EmailString( "a@gmail.com".into() ) ),
      EmailString::try_new( email )
    );
  }

  #[ test ]
  fn invalid_email()
  {
    assert_eq!
    ( 
      Err( InvalidEmail ),
      EmailString::try_new( "agmail.com" )
    );

    assert_eq!
    ( 
      Err( InvalidEmail ),
      EmailString::try_new( "some.email_address5@@gmail.com" )
    );
  }

  #[ test ]
  fn random_gets_all_values()
  {
    let rand = Random::new( 1, 2, 3 );
    for _ in 0..5
    {
      assert!( rand.values.contains( &*rand ) );
    }

    let rand = Random::new( "first", "second", "third" );
    for _ in 0..5
    {
      assert!( rand.values.contains( &*rand ) );
    }
  }
}