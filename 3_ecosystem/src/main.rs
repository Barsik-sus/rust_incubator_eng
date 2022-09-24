use std::io;

use actix_cors::Cors;
use actix_web::{HttpServer, App, web::Data, middleware};


mod db;
mod handlers;
mod schemas;

use self::{ db::get_pool, handlers::register };

#[ actix_web::main ]
async fn main() -> io::Result< () >
{
  env_logger::init_from_env( env_logger::Env::new().default_filter_or( "info" ) );
  
  let pool = get_pool();

  log::info!( "Application started" );
  HttpServer::new( move | |
  {
    App::new()
    .app_data( Data::new( pool.clone() ) )
    .configure( register )
    .wrap( Cors::permissive() )
    .wrap( middleware::Logger::default() )
  })
  .workers( 1 )
  .bind( "localhost:8081" )?
  .run()
  .await
}
