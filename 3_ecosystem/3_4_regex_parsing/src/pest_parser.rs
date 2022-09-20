use crate::*;
use pest::Parser;
use pest_derive::Parser;

#[ derive( Parser ) ]
#[ grammar = "grammar.pest" ]
struct GParser;

pub fn parse( input : &str ) -> ( Option< Sign >, Option< usize >, Option< Precision > )
{
  let pairs = GParser::parse( Rule::format_spec, input ).unwrap().next().unwrap().into_inner();
  let ( mut sign, mut width, mut precision ) = ( None, None, None );
  for pair in pairs
  {
    match pair.as_rule()
    {
      Rule::sign => sign = 
      {
        match pair.as_str()
        {
          "+" => Some( Sign::Plus ),
          "-" => Some( Sign::Minus ), 
          _ => None
        }
      },
      Rule::width => width = Some( pair.as_str().parse().unwrap() ),
      Rule::precision =>
      {
        precision = match pair.as_str()
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
      },
      _ => {}
    }
  }
  ( sign, width, precision )
}
