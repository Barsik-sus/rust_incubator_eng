use core::ptr::NonNull;
use std::sync::{ Arc, Mutex };

#[ derive( Debug ) ]
struct DoublyLinkedNode< T >
{
  data : T,
  next : Option< NonNull< DoublyLinkedNode< T > > >,
  prev : Option< NonNull< DoublyLinkedNode< T > > >,
}

#[ derive( Debug ) ]
struct DoublyLinkedList< T >
{
  first : Option< NonNull< DoublyLinkedNode< T > > >,
  last : Option< NonNull< DoublyLinkedNode< T > > >,
}

impl< T > DoublyLinkedList< T >
{
  fn new() -> Self
  {
    Self { first: None, last: None }
  }

  fn push_front( &mut self, value : T )
  {
    let new_node = Some
    (
      Box::leak
      (
        Box::new
        (
          DoublyLinkedNode{ data : value, next : self.first, prev : None }
        )
      ).into()
    );
    match self.first
    {
      None => self.last = new_node,
      Some( first ) =>
      {
        unsafe{ ( *first.as_ptr() ).prev = new_node }
      }
    }
    self.first = new_node
  }

  fn pop_front( &mut self ) -> Option< T >
  {
    self.first.map( | node |
    unsafe
    {
      let node = Box::from_raw( node.as_ptr() );
      self.first = node.next;

      match self.first
      {
        None => self.last = None,
        Some( first ) => ( *first.as_ptr() ).prev = None,
      }
      node.data
    })
  }

  fn push_back( &mut self, value : T )
  {
    let new_node = Some
    (
      Box::leak
      (
        Box::new
        (
          DoublyLinkedNode{ data : value, next : None, prev : self.last }
        )
      ).into()
    );
    match self.last
    {
      None => self.first = new_node,
      Some( mut last ) => unsafe{ last.as_mut().next = new_node; }
    }
    self.last = new_node;
  }

  fn pop_back( &mut self ) -> Option< T >
  {
    self.last.map( | node |
    unsafe
    {
      let node = Box::from_raw( node.as_ptr() );
      self.last = node.prev;

      match self.last
      {
        None => self.first = None,
        Some( last ) => ( *last.as_ptr() ).next = None
      }
      node.data
    })
  }
}

impl< T > Drop for DoublyLinkedList< T >
{
  fn drop(&mut self)
  {
    while let Some( node ) = self.pop_back()
    {
      drop( node )
    }
  }
}

struct DoublyLinkedListAsync< T >( Arc< Mutex< DoublyLinkedList< T > > > );

impl< T > DoublyLinkedListAsync< T >
{
  fn new() -> Self
  {
    Self( Arc::new( Mutex::new( DoublyLinkedList::new() ) ) )
  }

  fn push_front( &self, value : T )
  {
    self.0.lock().unwrap().push_front( value )
  }

  fn pop_front( &self ) -> Option< T >
  {
    self.0.lock().unwrap().pop_front()
  }

  fn push_back( &self, value : T )
  {
    self.0.lock().unwrap().push_back( value )
  }

  fn pop_back( &self ) -> Option< T >
  {
    self.0.lock().unwrap().pop_back()
  }
}

unsafe impl< T > Send for DoublyLinkedListAsync< T > {}
unsafe impl< T > Sync for DoublyLinkedListAsync< T > {}

fn main() {}


#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn not_thread_safe()
  {
    let mut list = DoublyLinkedList::new();
    list.push_back( 2 );
    assert_eq!( Some( 2 ), list.pop_back() );
    assert_eq!( None, list.pop_back() );

    list.push_back( 3 );
    assert_eq!( Some( 3 ), list.pop_front() );

    list.push_front( 2 );
    assert_eq!( Some( 2 ), list.pop_front() );

    list.push_front( 3 );
    assert_eq!( Some( 3 ), list.pop_back() );

    list.push_front( 4 );
    list.push_back( 5 );
    assert_eq!( Some( 4 ), list.pop_front() );
    assert_eq!( Some( 5 ), list.pop_front() );

    list.push_front( 4 );
    list.push_back( 5 );
    assert_eq!( Some( 4 ), list.pop_front() );
    list.push_front( 7 );
    list.push_back( 8 );
    assert_eq!( Some( 7 ), list.pop_front() );
    assert_eq!( Some( 5 ), list.pop_front() );
    assert_eq!( Some( 8 ), list.pop_front() );
  }

  #[ test ]
  fn thread_safe()
  {
    const COUNT : usize = 10;
    let list = DoublyLinkedListAsync::new();
    std::thread::scope( | s |
    {
      s.spawn( | |
      {
        for _ in 0..COUNT
        {
          list.push_back( 7 );
        }
      });
      s.spawn( | |
      {
        let mut count = 0;
        std::thread::sleep( std::time::Duration::from_millis( 10 ) );
        while let Some( _ ) = list.pop_front()
        {
          count += 1;
        }
        assert_eq!( COUNT, count );
      });
    });
  }
}
