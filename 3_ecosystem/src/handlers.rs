use actix_web::{ web::{ Data, self }, HttpResponse, Responder, route, get };
use actix_web_lab::respond::Html;
use juniper::http::{ GraphQLRequest, graphiql::graphiql_source };

use crate::{ schemas::root::{ create_schema, Schema, Context }, db::DbPool };


#[ get( "/graphiql" ) ]
async fn graphql_playgraund() -> impl Responder
{
  Html( graphiql_source( "/graphql", None ) )
}

#[ route( "/graphql", method = "GET", method = "POST" ) ]
async fn graphql
(
  pool : web::Data< DbPool >,
  schema : web::Data< Schema >,
  data : web::Json< GraphQLRequest >
) -> impl Responder
{
  let ctx = Context
  {
    db_pool : pool.get_ref().to_owned(),
  };
  let user = data.execute( &schema, &ctx ).await;
  HttpResponse::Ok().json( user )
}

pub fn register( config : &mut web::ServiceConfig )
{
  config
  .app_data( Data::new( create_schema() ) )
  .service( graphql )
  .service( graphql_playgraund );
}