use std::marker::PhantomData;

struct New;
struct Unmoderated;
struct Published;
struct Deleted;

mod post
{
  pub struct Title( pub String );
  pub struct Body( pub String );
}

pub struct Post< Status >
{
  pub title : post::Title,
  pub body : post::Body,
  status : PhantomData< Status >
}

impl Post< New >
{
  pub fn new( title : String, body : String ) -> Self
  {
    Post
    {
      title : post::Title( title ),
      body : post::Body( body ),
      status : PhantomData
    }
  }
}

impl Post< New >
{
  pub fn publish( self ) -> Post< Unmoderated >
  {
    Post { title : self.title, body : self.body, status : PhantomData }
  }
}

impl Post< Unmoderated >
{
  pub fn allow( self ) -> Post< Published >
  {
    Post { title : self.title, body : self.body, status : PhantomData }
  }

  pub fn deny( self ) -> Post< Deleted >
  {
    Post { title : self.title, body : self.body, status : PhantomData }
  }
}

impl Post< Published >
{
  pub fn delete( self ) -> Post< Deleted >
  {
    Post { title : self.title, body : self.body, status : PhantomData }
  }
}


fn main() {}


#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn allow_and_delete()
  {
    let post = Post::new( "Hi".into(), "World".into() );
    post.publish().allow().delete();
  }

  #[ test ]
  fn publish_and_deny()
  {
    let post = Post::new( "Hi".into(), "World".into() );
    post.publish().deny();
  }

  #[ test ]
  fn trybuild_tests()
  {
    let test = trybuild::TestCases::new();
    test.compile_fail( "tests/*.rs" );
  }
}