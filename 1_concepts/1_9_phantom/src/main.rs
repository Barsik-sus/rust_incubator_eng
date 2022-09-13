use std::marker::PhantomData;

use rand::seq::SliceRandom;

trait Facts
{
  fn facts() -> Vec< String >;
}

struct Fact< T >
{
  facts : Vec< String >,
  _type : PhantomData< T >
}

impl< T : Facts > Fact< T >
{
  fn new() -> Self
  {
    Self
    {
      facts: T::facts(),
      _type: PhantomData
    }
  }

  fn fact( &self ) -> String
  {
    self.facts.choose( &mut rand::thread_rng() ).unwrap().to_string()
  }
}

impl< T > Facts for Vec< T >
{
  fn facts() -> Vec< String >
  {
    vec!
    [
      "Some facts about Vec< T >".into(),
      "Second fact about Vec< T >".into()
    ]
  }
}

impl Facts for String
{
  fn facts() -> Vec< String >
  {
    vec!
    [
      "Some facts about String".into(),
      "Second fact about String".into()
    ]
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn facts_about_vec()
  {
    let v = Fact::< Vec< i32 > >::new();
    assert!( Vec::< i32 >::facts().contains( &v.fact() ) );
    assert!( Vec::< i32 >::facts().contains( &v.fact() ) );
  }

  #[ test ]
  fn facts_about_string()
  {
    let v = Fact::< String >::new();
    assert!( String::facts().contains( &v.fact() ) );
    assert!( String::facts().contains( &v.fact() ) );
  }
}
