use regex::Regex;
use lazy_static::lazy_static;
use crate::*;

lazy_static!
{
  static ref REGEX : Regex = Regex::new
  (
    r"((?P<fill>\w)?(?P<align>[<^>]))?(?P<sign>[+-])?[#]?[0]?(?P<width>\d+)?([.](?P<precision>([*]|(?P<argument>(\d+[$]))|\d+)))?(?P<type>[? ])?"
  ).unwrap();
}

fn take_sign_from_captures( captures : &regex::Captures ) -> Option< Sign >
{
  captures.name( "sign" ).and_then( | symbol |
  {
    match symbol.as_str()
    {
      "+" => Some( Sign::Plus ),
      "-" => Some( Sign::Minus ),
      _ => None
    }
  })
}

fn take_width_from_captures( captures : &regex::Captures ) -> Option< usize >
{
  captures.name( "width" ).and_then( | numbers | Some( numbers.as_str().parse().unwrap() ) )
}

fn take_precision_from_captures( captures : &regex::Captures ) -> Option< Precision >
{
  captures.name( "precision" ).and_then( | something |
  {
    match something.as_str()
    {
      "*" => Some( Precision::Asterisk ),
      string if string.chars().next_back().unwrap() == '$' => 
      {
        let mut number = string.chars();
        number.next_back();
        let argument = number.as_str().parse().unwrap();
        Some( Precision::Argument( argument ) )
      },
      string =>
      {
        string.parse().and_then( | numbers | Ok( Precision::Integer( numbers ) ) ).ok()
      }
    }
  })
}

pub fn parse( input : &str ) -> ( Option< Sign >, Option< usize >, Option< Precision > )
{
  if let Some( captures ) = REGEX.captures( input )
  {
    ( 
      take_sign_from_captures( &captures ),
      take_width_from_captures( &captures ),
      take_precision_from_captures( &captures )
    )
  }
  else
  {
    ( None, None, None )
  }
}
