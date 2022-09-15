use std::{ rc::Rc, cell::Cell };


struct OnlySync< T : Sync >( Rc< T > );

impl< T : Sync > OnlySync< T >
{
  fn new( val : T ) -> Self
  {
    Self( Rc::new( val ) )
  }

  fn this_is_threadsafe( &self ) -> &T
  {
    &self.0
  }
}

unsafe impl< T > Sync for OnlySync< T > 
where T : Sync {}

struct OnlySend< T : Send >( Cell< T > );

impl< T: Send > OnlySend< T >
{
  fn new( val : T ) -> Self
  {
    Self( Cell::new( val ) )
  }
}

struct SyncAndSend< T : Sync + Send >( T );

struct NotSyncNotSend< T >( Rc< T > );

impl< T > NotSyncNotSend< T >
{
  fn new( val : T ) -> Self
  {
    Self( Rc::new( val ) )
  }
}


fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn syncs()
  {
    let y_e = SyncAndSend( 8 );
    let y_ne = OnlySync::new( 8 );
    let _ny_e = OnlySend::new( 8 );
    let _ny_ne = NotSyncNotSend::new( 8 );
    std::thread::scope( | scope |
    {
      scope.spawn( | |
      {
        let _ = y_e.0;
        y_ne.this_is_threadsafe();
        // _ny_e.0.take(); // Cannot be shared between threads safely
        // *_ny_ne.0; // Cannot be shared between threads safely
      });
    });
  }

  #[ test ]
  fn sends()
  {
    let y_e = SyncAndSend( 8 );
    let _y_ne = OnlySync::new( 8 );
    let ny_e = OnlySend::new( 8 );
    let _ny_ne = NotSyncNotSend::new( 8 );
    std::thread::spawn( move | |
    {
      let _ = y_e.0;
      // _y_ne.this_is_threadsafe(); // Cannot be sent between threads safely
      ny_e.0.take();
      // *_ny_ne.0; // Cannot be sent between threads safely
    });
  }
}