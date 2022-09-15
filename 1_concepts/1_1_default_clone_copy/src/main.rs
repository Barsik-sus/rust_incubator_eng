use std::slice::SliceIndex;

#[ derive( Debug, Default, Copy, Clone, PartialEq ) ]
struct Point
{
  x : f32,
  y : f32,
}

#[ derive( Debug, PartialEq ) ]
struct EmptyPolyline;

#[ derive( Debug, Clone, PartialEq ) ]
struct Polyline( Vec< Point > );

impl TryFrom< Vec< Point > > for Polyline
{
  type Error = EmptyPolyline;

  fn try_from( points: Vec< Point > ) -> Result< Self, Self::Error >
  {
    if points.is_empty()
    {
      return Err( EmptyPolyline )
    }
    Ok( Self( points ) )
  }
}

impl< I : SliceIndex< [ Point ] > > std::ops::Index< I > for Polyline
{
  type Output = I::Output;

  fn index( &self, index: I ) -> &Self::Output
  {
    self.0.index( index )
  }
}

impl Polyline
{
  fn push( &mut self, value : Point )
  {
    self.0.push( value );
  }

  fn try_pop( &mut self ) -> Result< Point, EmptyPolyline >
  {
    if self.0.len() > 1
    {
      return Ok( self.0.pop().unwrap() )
    }
    Err( EmptyPolyline )
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests {
  use super::*;

  #[ test ]
  fn default_point()
  {
    let point = Point::default();
    assert_eq!( Point{ x : 0.0, y : 0.0 }, point );
  }

  #[ test ]
  fn copy_point() 
  {
    let point = Point{ x : 16.0, y : -2.9 };
    let copy_point = point;
    assert_eq!( point, copy_point );
  }

  #[ test ]
  #[ should_panic ]
  fn can_not_create_empty_polyline()
  {
    Polyline::try_from( vec![] ).unwrap();
  }

  #[ test ]
  fn copy_polyline()
  {
    let polyline = Polyline::try_from(
      vec![ Point::default(), Point::default() ]
    ).unwrap();
    let copy_polyline = polyline.clone();
    assert_eq!( polyline, copy_polyline );
  }

  #[ test ]
  fn non_default_polyline()
  {
    // fails at compile time cuz we don't want to use the Default
    // let polyline = Polyline::default();
  }

  #[ test ]
  fn push_to_polyline()
  {
    let mut polyline = Polyline::try_from
    (
      vec![ { Point{ x : 1.0, y : 1.0 } }, Point{ x : 2.0, y : 2.0 } ]
    ).unwrap();
    polyline.push( Point{ x : 3.0, y : 3.0 } );
    assert_eq!
    (
      vec!
      [
        Point{ x : 1.0, y : 1.0 },
        Point{ x : 2.0, y : 2.0 },
        Point{ x : 3.0, y : 3.0 }
      ],
      polyline[ .. ]
    )
  }

  #[ test ]
  fn try_pop_from_polyline_with_two_points()
  {
    let mut polyline = Polyline::try_from
    (
      vec![ { Point{ x : 1.0, y : 1.0 } }, Point{ x : 2.0, y : 2.0 } ]
    ).unwrap();

    assert_eq!( Ok( Point{ x : 2.0, y : 2.0 } ), polyline.try_pop() );

    assert_eq!
    (
      vec![ { Point{ x : 1.0, y : 1.0 } } ],
      polyline[ .. ]
    )
  }

  #[ test ]
  fn try_pop_from_polyline_with_one_point()
  {
    let mut polyline = Polyline::try_from
    (
      vec![ { Point{ x : 1.0, y : 1.0 } } ]
    ).unwrap();

    assert!( polyline.try_pop().is_err() );
    assert_eq!
    (
      vec![ { Point{ x : 1.0, y : 1.0 } } ],
      polyline[ .. ]
    )
  }
}
