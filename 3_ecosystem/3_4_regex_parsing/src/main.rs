mod regex_parser;
mod pest_parser;

fn main() {}


#[ derive( Debug, PartialEq ) ]
pub enum Sign
{
  Plus,
  Minus,
}

#[ derive( Debug, PartialEq ) ]
pub enum Precision
{
  Integer( usize ),
  Argument( usize ),
  Asterisk,
}

#[ cfg( test ) ]
mod spec
{
  use super::*;

  #[ test ]
  fn parses_sign()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", None ),
      ( ">+8.*", Some( Sign::Plus ) ),
      ( "-.1$x", Some( Sign::Minus ) ),
      ( "a^#043.8?", None ),
    ]
    {
      let ( sign, .. ) = regex_parser::parse( input );
      assert_eq!( sign, expected, "Regex parser" );
      let ( sign, .. ) = pest_parser::parse( input );
      assert_eq!( sign, expected, "Pest parser" );
    }
  }

  #[ test ]
  fn parses_width()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", Some( 8 ) ),
      ( ">+8.*", Some( 8 ) ),
      ( "-.1$x", None ),
      ( "a^#043.8?", Some( 43 ) ),
    ]
    {
      let ( _, width, _ ) = regex_parser::parse( input );
      assert_eq!( width, expected, "Regex parser" );
      let ( _, width, _ ) = pest_parser::parse( input );
      assert_eq!( width, expected, "Pest parser" );
    }
  }

  #[ test ]
  fn parses_precision()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", Some( Precision::Asterisk ) ),
      ( ">+8.*", Some( Precision::Asterisk ) ),
      ( "-.1$x", Some( Precision::Argument( 1 ) ) ),
      ( "a^#043.8?", Some( Precision::Integer( 8 ) ) ),
    ]
    {
      let ( _, _, precision ) = regex_parser::parse( input );
      assert_eq!( precision, expected, "Regex parser" );
      let ( _, _, precision ) = pest_parser::parse( input );
      assert_eq!( precision, expected, "Pest parser" );
    }
  }
}
