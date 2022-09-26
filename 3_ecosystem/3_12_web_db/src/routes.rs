use actix_web::{ get, post, delete, web, HttpResponse, Responder };
use diesel::{ r2d2::Pool, r2d2::ConnectionManager, SqliteConnection };

use crate::{ ID, db::{ IArticle, self } };

type DbPool = Pool< ConnectionManager< SqliteConnection > >;


// Returns all articles
#[ get( "/articles" ) ]
pub async fn get_articles( db_conn : web::Data< DbPool > ) -> actix_web::Result< impl Responder >
{
  let articles = db::get_articles( &mut db_conn.get().unwrap() ).unwrap();
  Ok( web::Json( articles ) )
}

// Returns an article by its ID
#[ get( "/article/{id}" ) ]
pub async fn get_article( db_conn : web::Data< DbPool >, a_id : web::Path< ( ID, ) > ) -> actix_web::Result< impl Responder >
{
  let article = db::get_article( &mut db_conn.get().unwrap(), a_id.into_inner().0 ).unwrap();
  Ok( web::Json( article ) )
}

// Posts an article. The article must be with labels
#[ post( "/article" ) ]
pub async fn post_article( db_conn : web::Data< DbPool >, article : web::Json< IArticle > ) -> actix_web::Result< impl Responder >
{
  let id = db::insert_article( &mut db_conn.get().unwrap(), article.0 ).unwrap();
  Ok( web::Json( id ) )
}

// Removes an article by its ID
#[ delete( "/article/{id}" ) ]
pub async fn delete_article( db_conn : web::Data< DbPool >, article_id : web::Path< ( ID, ) > ) -> actix_web::Result< impl Responder >
{
  db::delete_article( &mut db_conn.get().unwrap(), article_id.0 ).unwrap();
  Ok( HttpResponse::Ok() )
}
