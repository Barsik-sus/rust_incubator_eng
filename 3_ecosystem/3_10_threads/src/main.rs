use crossbeam_channel::{ Receiver, Sender, RecvError };
use rand::{ Rng, Fill };
use rayon::prelude::*;

#[ derive( Debug ) ]
pub struct ProducerCantProduce;

#[ derive( Debug ) ]
pub struct Producer< T >
{
  matrix : [ T; 4096 ],
  channel : Sender< [ T; 4096 ] >
}

impl< T : Clone > Producer< T > 
where [ T ] : Fill
{
  pub fn produce( &mut self ) -> Result< (), ProducerCantProduce >
  {
    rand::thread_rng().fill( &mut self.matrix );
    self.channel.send( self.matrix.clone() ).or( Err( ProducerCantProduce ) )
  }
}

#[ derive( Debug ) ]
pub struct Consumer< T >
{
  number : i8,
  chanel : Receiver< [ T; 4096 ] >
}

impl Consumer< u8 >
{
  fn sum( &self, data : &[ u8 ] ) -> u128
  {
    data.par_iter().map( | &i | i as u128 ).sum()
  }

  pub fn consume( &self ) -> Result< (), RecvError >
  {
    println!( "Consumer {} : {}", self.number, self.sum( &self.chanel.recv()? ) );
    Ok( () )
  }
}

fn main()
{
  let ( s, r ) = crossbeam_channel::bounded::< [ u8; 4096 ] >( 2 );

  let consumer_1 = r.clone();
  std::thread::spawn( | | 
  {
    let consumer = Consumer{ number : 1, chanel : consumer_1 };
    while let Ok( () ) = consumer.consume() {}
  });

  std::thread::spawn( | | 
  {
    let consumer = Consumer{ number : 2, chanel : r };
    while let Ok( () ) = consumer.consume() {}
  });

  let mut producer = Producer{ channel : s, matrix : [ 0; 4096 ] };
  while let Ok( () ) = producer.produce() {}
}
