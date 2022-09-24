use diesel::prelude::*;
use juniper::{  GraphQLObject, GraphQLInputObject };


use super::root::Context;

mod models;
pub mod schema;

use models::*;

pub type ID = i32;


#[ derive( GraphQLObject ) ]
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
}

impl User
{
  pub fn get_user_by_id( ctx : &Context, uid : ID ) -> QueryResult< Self >
  {
    use crate::schemas::user::schema::users::dsl::users;
    use crate::schemas::user::schema::friendship::dsl::*;

    let mut conn = ctx.db_pool.get().unwrap();
    let m_user : MUser = users.find( uid ).first( &mut conn )?;
    let friends = friendship.select( friend_id ).filter( user_id.eq( uid ) )
    .load( &mut conn )?.into_iter()
    .map( | fuid | Self::get_user_by_id( ctx, fuid ).unwrap() )
    .collect();
    Ok( User{ id : uid, name : m_user.name, friends } )
  }
}
