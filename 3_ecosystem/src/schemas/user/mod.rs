use diesel::prelude::*;
use juniper::{  GraphQLObject, GraphQLInputObject };

use super::context::Context;

mod models;
pub mod schema;

use models::*;

pub type ID = i32;


#[ derive( Debug, GraphQLObject, Clone ) ]
pub struct User
{
  pub id : ID,
  pub name : String,
  pub friends : Vec< User >
}

#[ derive( GraphQLInputObject ) ]
pub struct NewUser
{
  pub name : String,
  pub password : String,
}

impl User
{
  pub fn new( ctx : &Context, new_user : NewUser ) -> QueryResult< User >
  {
    use crate::schemas::user::schema::users::dsl::users;
    use crate::schemas::user::schema::user_password::dsl::*;

    let mut conn = ctx.db_pool().get().unwrap();

    let m_user = NewMUser{ name : new_user.name };
    let m_user = diesel::insert_into( users )
    .values( &m_user )
    .get_result::< MUser >( &mut conn ).unwrap();

    let m_user_password = UserPassword{ user_id : m_user.id, password : new_user.password };
    diesel::insert_into( user_password )
    .values( &m_user_password )
    .execute( &mut conn ).unwrap();

    let user = User
    {
      id : m_user.id,
      name : m_user.name,
      friends : vec![]
    };

    Ok( user )
  }

  pub fn get_user_by_id( ctx : &Context, uid : ID ) -> QueryResult< Self >
  {
    use crate::schemas::user::schema::users::dsl::users;
    use crate::schemas::user::schema::friendship::dsl::*;

    let mut conn = ctx.db_pool().get().unwrap();
    let m_user : MUser = users.find( uid ).first( &mut conn )?;
    let friends = friendship.select( friend_id ).filter( user_id.eq( uid ) )
    .load( &mut conn )?.into_iter()
    .map( | fuid | Self::get_user_by_id( ctx, fuid ).unwrap() )
    .collect();
    Ok( User{ id : uid, name : m_user.name, friends } )
  }

  pub fn get_friends( &self, ctx : &Context ) -> QueryResult< Vec< User > >
  {
    Ok( Self::get_user_by_id( ctx, self.id )?.friends )
  }

  pub fn insert_pair_friends( ctx : &Context, uid : ID, fid : ID ) -> QueryResult< User >
  {
    use crate::schemas::user::schema::friendship::dsl::*;

    let mut conn = ctx.db_pool().get().unwrap();

    let friends = NewMFriendship{ user_id : uid, friend_id : fid };
    diesel::insert_into( friendship )
    .values( friends )
    .execute( &mut conn ).unwrap();
    User::get_user_by_id( ctx, uid )
  }

  pub fn add_to_friends( &self, ctx : &Context, fid : ID ) -> QueryResult< User >
  {
    Self::insert_pair_friends( ctx, self.id, fid )
  }

  pub fn delete_pair_friends( ctx : &Context, uid : ID, fid : ID ) -> QueryResult< User >
  {
    use crate::schemas::user::schema::friendship::dsl::*;

    let mut conn = ctx.db_pool().get().unwrap();

    diesel::delete( friendship )
    .filter( user_id.eq( uid ).and( friend_id.eq( fid ) ) )
    .execute( &mut conn ).unwrap();
    User::get_user_by_id( ctx, uid )
  }

  pub fn delete_friend( &self, ctx : &Context, fid : ID ) -> QueryResult< User >
  {
    Self::delete_pair_friends( ctx, self.id, fid )
  }

  fn find_user_by_name( ctx : &Context, username : String ) -> QueryResult< MUser >
  {
    use crate::schemas::user::schema::users;

    let mut conn = ctx.db_pool().get().unwrap();
    let m_user : MUser = users::dsl::users
    .filter( users::name.eq( username ) )
    .first( &mut conn )?;

    Ok( m_user )
  }

  pub fn check( ctx : &Context, username : String, password : String ) -> QueryResult< ID >
  {
    use crate::schemas::user::schema::user_password;

    let mut conn = ctx.db_pool().get().unwrap();
    let user = User::find_user_by_name( ctx, username )?;

    let up : UserPassword = user_password::dsl::user_password
    .filter
    (
      user_password::user_id.eq( user.id )
      .and( user_password::password.eq( password ) )
    )
    .first( &mut conn )?;

    Ok( up.user_id )
  }
}
