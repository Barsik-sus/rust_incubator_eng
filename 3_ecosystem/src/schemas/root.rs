use juniper::{ FieldResult, RootNode, EmptySubscription };

use super::user::{ User, NewUser, ID };
use crate::db::DbPool;


pub struct Context
{
  pub db_pool : DbPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[ juniper::graphql_object( Context = Context ) ]
impl QueryRoot
{
  fn user_friends( ctx : &Context, user_id : ID ) -> FieldResult< User >
  {
    Ok( User::get_user_by_id( ctx, user_id )? )
  }
}

pub struct MutationRoot;

#[ juniper::graphql_object( Context = Context ) ]
impl MutationRoot
{
  fn create_user( _ctx : &Context, new_user : NewUser ) -> FieldResult< User >
  {
    Ok( User
    {
      id : 22,
      name : new_user.name,
      friends : vec![],
    })
  }

  // fn add_to_friend( _ctx : &Context, _user_id : ID, _friend_id : ID ) -> FieldResult< () >
  // {
  //   Ok( () )
  // }
}

pub type Schema = RootNode< 'static, QueryRoot, MutationRoot, EmptySubscription< Context > >;

pub fn create_schema() -> Schema
{
  Schema::new( QueryRoot{}, MutationRoot{}, EmptySubscription::new() )
}