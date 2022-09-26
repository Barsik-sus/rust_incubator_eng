use std::env;

use diesel::{ pg::PgConnection, r2d2::ConnectionManager };
use dotenv::dotenv;
use r2d2::Pool;

pub type DbPool = Pool< ConnectionManager< PgConnection > >;

pub fn get_pool() -> DbPool
{
  dotenv().ok();
  let url = env::var( "DATABASE_URL" ).expect( "Please set \"DATABASE_URL\" variable in your environment" );
  let manager = ConnectionManager::< PgConnection >::new( url );
  r2d2::Pool::builder()
  .build( manager )
  .expect( "Could not build connection pool" )
}