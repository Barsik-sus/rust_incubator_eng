fn main()
{
  let mut s = Solver
  {
    expected : Trinity { a : 1, b : 2, c : 3 },
    unsolved : vec![
      Trinity { a : 1, b : 2, c : 3 },
      Trinity { a : 2, b : 1, c : 3 },
      Trinity { a : 2, b : 3, c : 1 },
      Trinity { a : 3, b : 1, c : 2 },
    ],
  };
  s.resolve();
  println!( "{:?}", s )
}

#[ derive( Clone, Debug, PartialEq ) ]
struct Trinity< T >
{
  a : T,
  b : T,
  c : T,
}

impl< T > Trinity< T >
{
  fn rotate( &mut self )
  {
    std::mem::swap( &mut self.a, &mut self.c );
    std::mem::swap( &mut self.a, &mut self.b );
  }
}

#[ derive( Debug ) ]
struct Solver< T >
{
  expected : Trinity< T >,
  unsolved : Vec< Trinity< T > >,
}

impl< T : PartialEq > Solver< T >
{
  fn resolve( &mut self )
  {
    let mut unsolved = Vec::< Trinity< T > >::with_capacity( self.unsolved.len() );
    'l : for mut t in std::mem::take( &mut unsolved ).into_iter()
    {
      for _ in 0 .. 3
      {
        if t == self.expected
        {
          continue 'l;
        }
        t.rotate();
      }
      unsolved.push( t )
    }
    self.unsolved = unsolved;
  }
}
