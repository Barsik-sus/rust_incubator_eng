use std::path::Path;
use rand::seq::SliceRandom;
use sha3::{ Sha3_256, Digest };
use argon2::
{
  password_hash::
  {
      rand_core::OsRng,
      PasswordHasher, SaltString, self
  },
  Argon2
};

const CHAR_SET : [ char ; 62 ] = 
[
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];


#[ inline ]
fn generate_password( symbols : &[ char ], length : usize ) -> String
{
  ( 0..( if symbols.len() > 0 { length } else { 0 } ) ).fold
  (
    String::with_capacity( length ),
    | mut acc, _ |
    {
      acc.push( *symbols.choose( &mut rand::thread_rng() ).unwrap() );
      acc
    } 
  )
}


#[ inline ]
fn select_rand_val< T : Clone >( values : &[ T ] ) -> Option< &T >
{
  values.choose( &mut rand::thread_rng() )
}

#[ inline ]
fn new_access_token() -> String
{
  ( 0..64 ).fold
  (
    String::with_capacity( 64 ),
    | mut acc, _ |
    {
      acc.push( *CHAR_SET.choose( &mut rand::thread_rng() ).unwrap() );
      acc
    } 
  )
}

fn get_file_hash< P >( path : P ) -> std::io::Result< Vec< u8 > >
where
  P : AsRef< Path >
{
  let mut hasher = Sha3_256::new();
  let data = std::fs::read( path )?;
  hasher.update( data );
  Ok( hasher.finalize().to_vec() )
}

fn hash_password< S >( password : S ) -> password_hash::Result< String >
where
  S : AsRef< [ u8 ] >
{
  let salt = SaltString::generate( &mut OsRng );
  let argon2 = Argon2::default();
  Ok( argon2.hash_password( password.as_ref(), &salt )?.to_string() )
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use argon2::{ PasswordVerifier, password_hash::PasswordHash };

  #[ test ]
  fn generate_password_test()
  {
    let symbols = [ 'a', 'b', 'd', '1', '2' ];
    let length = 3;
    let pass = generate_password( &symbols, length );
    assert_eq!( length, pass.len() );
    pass.chars().into_iter()
    .for_each( | c | assert!( symbols.contains( &c ) ) );
  }

  #[ test ]
  fn generate_password_empty_set_test()
  {
    assert_eq!( "", generate_password( &[], 10 ) );
  }

  #[ test ]
  fn select_rand_val_test()
  {
    let set = [ 1, 2, 3, 9 ];
    for _ in 0..5
    {
      let rv = select_rand_val( &set ).unwrap();
      assert!( set.contains( rv ) );
    }
  }

  #[ test ]
  fn select_rand_val_empty_set_test()
  {
    assert_eq!( None, select_rand_val( Vec::< i32 >::new().as_slice() ) );
  }

  #[ test ]
  fn access_token_test()
  {
    let token = new_access_token();
    assert_eq!( 64, token.len() );
    token.chars().into_iter()
    .for_each( | c | assert!( CHAR_SET.contains( &c ) ) );
  }

  #[ test ]
  fn file_haser_test()
  {
    assert!( get_file_hash( "./Cargo.toml" ).is_ok() );
    assert!( get_file_hash( "./" ).is_err() );
  }

  #[ test ]
  fn password_hash_test()
  {
    let password = b"gj3#@jc!lp]";
    let hash = hash_password( password ).unwrap();
    let parsed_hash = PasswordHash::new( &hash ).unwrap();
    assert!( Argon2::default().verify_password( password, &parsed_hash ).is_ok() );
  }
}