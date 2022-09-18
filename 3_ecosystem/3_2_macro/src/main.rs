use step_3_2::*;
use std::collections::BTreeMap;

#[ macro_export ]
macro_rules! btreemap
{
  ( $( $x : expr ),* ) =>
  {{
    let mut tmp_btreemap = BTreeMap::new();
    $(
      tmp_btreemap.insert( $x.0, $x.1 );
    )*
    tmp_btreemap
  }};
}

fn main() {}

#[ test ]
fn declarative_test()
{
  let mut tree = btreemap!( ( 1, "Hello" ), ( 2, "World!" ) );
  assert_eq!( Some( &"Hello" ), tree.get( &1 ) );
  assert_eq!( Some( &"World!" ), tree.get( &2 ) );
  tree.insert( 4, "Yep" );
  assert_eq!( Some( &"Yep" ), tree.get( &4 ) );
}

#[ test ]
fn procedural_test()
{
  let mut tree = pbtreemap!( ( 1, "Hello" ), ( 2, "World!" ) );
  assert_eq!( Some( &"Hello" ), tree.get( &1 ) );
  assert_eq!( Some( &"World!" ), tree.get( &2 ) );
  tree.insert( 4, "Yep" );
  assert_eq!( Some( &"Yep" ), tree.get( &4 ) );
}