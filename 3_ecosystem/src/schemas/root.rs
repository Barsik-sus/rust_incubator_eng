use juniper::{ FieldResult, RootNode, EmptySubscription };

use super::context::Context;
use super::user::{ User, NewUser, ID };


pub struct QueryRoot;

#[ juniper::graphql_object( Context = Context ) ]
impl QueryRoot
{
  fn user_friends( ctx : &Context, user_id : ID ) -> FieldResult< User >
  {
    // ctx.me().unwrap(); // * if the user must be authorized
    Ok( User::get_user_by_id( ctx, user_id )? )
  }

  async fn my_friends( ctx : &Context ) -> FieldResult< Vec< User > >
  {
    Ok( ctx.me().unwrap().get_friends( ctx )? )
  }
}

pub struct MutationRoot;

#[ juniper::graphql_object( Context = Context ) ]
impl MutationRoot
{
  fn create_user( ctx : &Context, new_user : NewUser ) -> FieldResult< User >
  {
    Ok( User::new( ctx, new_user )? )
  }

  fn login( ctx : &Context, username : String, password : String ) -> FieldResult< User >
  {
    let uid = User::check( ctx, username, password )?;
    let user = User::get_user_by_id( ctx, uid )?;
    ctx.authorize( user.clone() );
    log::info!( "User logged in" );
    Ok( user )
  }

  fn add_to_friends( ctx : &Context, friend_id : ID ) -> User
  {
    ctx.me().unwrap().add_to_friends( ctx, friend_id ).unwrap()
  }

  fn remove_friend( ctx : &Context, friend_id : ID ) -> User
  {
    ctx.me().unwrap().delete_friend( ctx, friend_id ).unwrap()
  }
}

pub type Schema = RootNode< 'static, QueryRoot, MutationRoot, EmptySubscription< Context > >;

pub fn create_schema() -> Schema
{
  Schema::new( QueryRoot{}, MutationRoot{}, EmptySubscription::new() )
}