use std::sync::{ Arc, Mutex };

use super::user::User;
use crate::db::DbPool;


pub struct Context
{
  pub db_pool : DbPool,
  pub user : Arc< Mutex< Option< User > > >,
}

impl juniper::Context for Context {}

impl Context
{
  pub fn db_pool( &self ) -> DbPool
  {
    self.db_pool.clone()
  }

  pub fn authorize( &self, user : User )
  {
    *self.user.lock().unwrap() = Some( user );
  }

  pub fn me( &self ) -> Option< User >
  {
    self.user.lock().unwrap().take()
  }
}