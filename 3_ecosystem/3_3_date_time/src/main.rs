use chrono::{ prelude::*, LocalResult };


fn main()
{
  println!( "Implement me!" );
}

const NOW : &str = "2019-06-26";

#[ derive( Debug, PartialEq ) ]
struct InvalidDate;

#[ derive( Debug, PartialEq ) ]
struct User
{
  birthdate : Date< Utc >,
}

impl User
{
  fn with_birthdate( year : i32, month : u32, day : u32 ) -> Result< Self, InvalidDate >
  {
    match Utc.ymd_opt( year, month, day )
    {
      LocalResult::Single( date ) => Ok( User{ birthdate : date } ),
      _ => Err( InvalidDate )
    }
  }

  /// Returns current age of [ `User`] in years.
  fn age( &self ) -> u16
  {
    let native_date = NaiveDate::parse_from_str( NOW, "%Y-%m-%d" ).unwrap();
    let date_time = Utc.from_local_date( &native_date ).unwrap();
    
    match date_time.years_since( self.birthdate )
    {
      None => 0,
      Some( years ) => years as u16
    }
  }

  /// Checks if [ `User`] is 18 years old at the moment.
  fn is_adult( &self ) -> bool
  {
    self.age() >= 18
  }
}

#[ cfg( test ) ]
mod age_spec
{
  use super::*;

  #[ test ]
  fn counts_age()
  {
    for ( ( y, m, d ), expected ) in vec!
    [
      ( ( 1990, 6, 4 ), 29 ),
      ( ( 1990, 7, 4 ), 28 ),
      ( ( 0, 1, 1 ), 2019 ),
      ( ( 1970, 1, 1 ), 49 ),
      ( ( 2019, 6, 25 ), 0 ),
      ( ( 2019, 6, 25 ), 0 ),
    ]
    {
      let user = User::with_birthdate( y, m, d ).unwrap();
      assert_eq!( user.age(), expected );
    }
  }

  #[ test ]
  fn zero_if_birthdate_in_future()
  {
    for ( ( y, m, d ), expected ) in vec!
    [
      ( ( 2032, 6, 25 ), 0 ),
      // ( ( 2016, 6, 27 ), 0 ), // ? is it correct test?
      ( ( 2019, 6, 27 ), 0 ),
      ( ( 3000, 6, 27 ), 0 ),
      ( ( 9999, 6, 27 ), 0 )
    ]
    {
      let user = User::with_birthdate( y, m, d ).unwrap();
      assert_eq!( user.age(), expected );
    }
  }

  #[ test ]
  fn adult()
  {
    for ( ( y, m, d ), expected ) in vec!
    [
      ( ( 1990, 6, 4 ), true ),
      ( ( 2001, 6, 24 ), true ),
      ( ( 2001, 6, 27 ), false ),
      ( ( 2019, 6, 25 ), false ),
    ]
    {
      let user = User::with_birthdate( y, m, d ).unwrap();
      assert_eq!( user.is_adult(), expected );
    }
  }

  #[ test ]
  fn invalid_date()
  {
    for ( year, month, day ) in vec!
    [
      ( 2000, 13, 1 ),
      ( 2000, 10, 32 ),
      ( 1988, 2, 30 )
    ]
    {
      assert_eq!( Err( InvalidDate ), User::with_birthdate( year, month, day ) );
    }
  }

}
