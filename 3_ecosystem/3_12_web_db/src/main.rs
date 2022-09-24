use actix_web::{ web, HttpServer, App };

type ID = i32;

mod db;
mod routes;

#[ actix_web::main ]
async fn main() -> std::io::Result< () >
{
  dbg!( "Application started" );
  let db_connection = web::Data::new( db::establish_connection() );

  HttpServer::new( move | |
  {
    App::new()
    .service( routes::get_article )
    .service( routes::get_articles )
    .service( routes::post_article )
    .service( routes::delete_article )
    .app_data( db_connection.clone() )
  })
  .bind(( "127.0.0.1", 8080 ))?
  .run()
  .await
}
