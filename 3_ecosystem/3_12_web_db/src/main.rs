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

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::db::IArticle;
  use actix_web::
  {
    web::{ self, Bytes },
    http::StatusCode,
    test, body::to_bytes,
  };

  #[ actix_web::test ]
  async fn some_test()
  {
    let db_connection = web::Data::new( db::establish_connection() );
    
    let app = test::init_service
    (
      App::new()
      .service( routes::get_article )
      .service( routes::get_articles )
      .service( routes::post_article )
      .service( routes::delete_article )
      .app_data( db_connection.clone() )
    ).await;

    let article = IArticle
    {
      title : "Test title".to_owned(),
      body : "Some body".to_owned(),
      labels : vec![ "label 1".to_owned() ]
    };

    // POST article
    let req = test::TestRequest::post()
    .uri( "/article" )
    .set_json( article )
    .to_request();

    let resp = test::call_service( &app, req ).await;
    assert_eq!( StatusCode::OK, resp.status() );
    let body = to_bytes( resp.into_body() ).await.unwrap();
    let id = body.as_str().to_string().parse::< i32 >().unwrap();

    // GET article by ID
    let req = test::TestRequest::get()
    .uri( format!( "/article/{id}" ).as_str() )
    .to_request();

    let resp = test::call_service( &app, req ).await;
    assert_eq!( StatusCode::OK, resp.status() );

    // GET all articles
    let req = test::TestRequest::get()
    .uri( "/articles" )
    .to_request();

    let resp = test::call_service( &app, req ).await;
    assert_eq!( StatusCode::OK, resp.status() );

    // DELETE article by ID
    let req = test::TestRequest::delete()
    .uri( format!( "/article/{id}" ).as_str() )
    .to_request();

    let resp = test::call_service( &app, req ).await;
    assert_eq!( StatusCode::OK, resp.status() );
  }

  trait BodyTest
  {
    fn as_str( &self ) -> &str;
  }

  impl BodyTest for Bytes
  {
    fn as_str( &self ) -> &str
    {
      std::str::from_utf8( self ).unwrap()
    }
  }
}