pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ Pool, ConnectionManager };
use dotenvy::dotenv;
use std::env;
use serde::{Serialize, Deserialize};

use crate::ID;
use self::models::*;

#[ derive( Serialize, Deserialize ) ]
pub struct Article
{
  id : ID,
  title : String,
  body : String,
  labels : Vec< String >
}

#[ derive( Serialize, Deserialize ) ]
pub struct IArticle
{
  title : String,
  body : String,
  labels : Vec< String >
}

pub fn establish_connection() -> Pool< ConnectionManager< SqliteConnection > >
{
  dotenv().ok();

  let database_url = env::var( "DATABASE_URL").expect( "DATABASE_URL must be set" );
  let manager = ConnectionManager::< SqliteConnection >::new( &database_url );
  Pool::builder().build( manager ).expect( "Failed to create pool" )
}

// Returns an article by its ID
// TODO: Better Error result
pub fn get_article( conn : &mut SqliteConnection, take_id : ID ) -> Result< Article, () >
{
  use schema::articles::dsl::*;

  let article : MArticle = articles.find( take_id ).first( conn ).or( Err( () ) )?;

  let rarticle = Article
  {
    id : article.id,
    title : article.title,
    body : article.body,
    labels : get_labels( conn, article.id ).or( Err( () ) )?,
  };
  Ok( rarticle )
} 

// Returns all articles
// TODO: Better Error result
pub fn get_articles( conn : &mut diesel::SqliteConnection ) -> Result< Vec< Article >, () >
{
  use schema::articles::dsl::*;

  let r_articles : Vec< MArticle > = articles.load( conn ).or( Err( () ) )?;

  let rarticles = r_articles.iter().map( | article |
  {
    Article
    {
      id : article.id,
      title : article.title.clone(),
      body : article.body.clone(),
      labels : get_labels( conn, article.id ).or( Err( () ) ).unwrap(),
    }
  })
  .collect();

  Ok( rarticles )
}

// Inserts an article. The article must be with labels
// TODO: Better Error result
pub fn insert_article( conn : &mut diesel::SqliteConnection, article : IArticle ) -> Result< ID, () >
{
  use schema::articles;

  let model_article = NewMArticle
  {
    title : article.title,
    body : article.body,
  };

  diesel::insert_into( articles::table )
  .values( &model_article )
  // .get_result( conn ) // ? Should work but it is not implemented for SQLite
  .execute( conn )
  .or( Err( () ) )?;

  let last = articles::table
  .order( articles::id.desc() )
  .select( articles::id )
  .first( conn )
  .or( Err( () ) )?;

  insert_labels( conn, last, article.labels )?;

  Ok( last )
}

// Returns a list of labels by article ID
// TODO: Better Error result
pub fn get_labels( conn : &mut diesel::SqliteConnection, article_id : ID ) -> Result< Vec< String >, () >
{
  use schema::labels;

  let labels = labels::dsl::labels.filter( labels::article_id.eq( article_id ) )
  .load::< MLabel >( conn )
  .or( Err( () ) )?
  .iter().map( | label | label.label.clone() )
  .collect();
  Ok( labels )
}

// Inserts a list of labels for an article by its ID
// TODO: Better Error result
pub fn insert_labels( conn : &mut diesel::SqliteConnection, article_id : ID, new_labels : Vec< String > ) -> Result< (), () >
{
  use schema::labels;

  let new_labels = new_labels.iter()
  .map( | label | NewMLabel{ label : label.clone(), article_id } ).collect::< Vec< _ > >();

  diesel::insert_into( labels::table )
  .values( &new_labels )
  .execute( conn )
  .or( Err( () ) )?;

  Ok( () )
}

// Removes an article by its ID
// TODO: Better Error result
pub fn delete_article( conn : &mut diesel::SqliteConnection, article_id : ID ) -> Result< (), () >
{
  use schema::articles;

  diesel::delete
  (
    articles::dsl::articles.filter( articles::id.eq( article_id ) )
  )
  .execute( conn )
  .or( Err( () ) )?;
  Ok( () )
}
