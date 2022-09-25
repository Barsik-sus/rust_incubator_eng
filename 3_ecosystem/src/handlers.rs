use std::sync::{ Arc, Mutex };

use actix_web::{ web::{ Data, self }, HttpResponse, Responder, route, get };
use actix_web_lab::respond::Html;
use juniper::http::{ GraphQLRequest, graphiql::graphiql_source };

use crate::
{
  schemas::
  {
    context::Context,
    root::{ create_schema, Schema }, user::User
  },
  db::DbPool
};


#[ get( "/graphiql" ) ]
async fn graphql_playgraund() -> impl Responder
{
  Html( graphiql_source( "/graphql", None ) )
}

#[ route( "/graphql", method = "GET", method = "POST" ) ]
async fn graphql
(
  user : web::Data< Arc< Mutex< Option< User > > > >,
  pool : web::Data< DbPool >,
  schema : web::Data< Schema >,
  data : web::Json< GraphQLRequest >
) -> impl Responder
{
  let ctx = Context
  {
    db_pool : pool.get_ref().to_owned(),
    user : Arc::clone( &user.get_ref().to_owned() ),
  };
  let user = data.execute( &schema, &ctx ).await;
  HttpResponse::Ok().json( user )
}

pub fn register( config : &mut web::ServiceConfig )
{
  config
  .app_data( Data::new( Arc::new( Mutex::new( None::< User > ) ) ) )
  .app_data( Data::new( create_schema() ) )
  .service( graphql )
  .service( graphql_playgraund );
}