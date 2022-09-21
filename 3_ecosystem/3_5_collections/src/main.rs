use im::HashMap;
use std::borrow::Cow;

pub trait UserRepository
{
  fn get( &self, id : usize ) -> Option< &User >;
  fn gets( &self, ids : &[ usize ] ) -> Vec< &User >;
  fn find( &self, username : &str ) -> Vec< usize >;
}

#[ derive( Debug, Clone, PartialEq ) ]
pub struct User
{
  pub id : usize,
  pub username : Cow< 'static, str >,
}

impl User
{
  fn new< S >( id : usize, username : S ) -> Self
  where
    S : Into< Cow< 'static, str > >
  {
    Self { id, username : username.into() }
  }
}

pub struct DB< K, V >( HashMap< K, V > );

impl DB< usize, User >
{
  pub fn new() -> Self
  {
    DB( HashMap::new() )
  }

  pub fn insert( &mut self, user : User ) -> Option< User >
  {
    self.0.insert( user.id, user )
  }
}

impl UserRepository for DB< usize, User >
{
  fn get( &self, id : usize ) -> Option< &User >
  {
    self.0.get( &id )
  }

  fn gets( &self, ids : &[ usize ] ) -> Vec< &User >
  {
    self.0.iter()
    .filter( | ( &id, _ ) | ids.contains( &id ) )
    .map( | ( _, user ) | user ).collect()
  }

  fn find( &self, username : &str ) -> Vec< usize >
  {
    self.0.iter()
    .filter( | ( _, user ) | user.username.contains( username ) )
    .map( | ( id, _ ) | *id ).collect()
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn all_in_one()
  {
    let mut db = DB::new();
    let ( user_1, user_2, user_3 ) = 
    (
      User::new( 1,"Barsik" ),
      User::new( 2,"Persik" ),
      User::new( 3,"Peter" ),
    );

    for user in [ &user_1, &user_2, &user_3 ]
    {
      db.insert( user.clone() );
    }
    for user in [ &user_1, &user_2, &user_3 ]
    {
      assert_eq!( Some( user ), db.get( user.id ) );
    }
    assert_eq!( None, db.get( 100 ) );

    assert_eq!
    (
      vec![ &user_1, &user_3 ],
      {
        let mut result = db.gets( &[ 1, 3 ] );
        result.sort_by_key( | user | user.id );
        result
      }
    );
    assert_eq!( Vec::< &User >::new(), db.gets( &[ 100 ] ) );

    assert_eq!
    (
      vec![ 1, 2 ],
      {
        let mut result = db.find( "sik" );
        result.sort();
        result
      }
    );
    assert_eq!( Vec::< usize >::new(), db.find( "666" ) );
  }
}
