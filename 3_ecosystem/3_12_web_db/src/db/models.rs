use diesel::prelude::*;
use super::schema::{ articles, labels };

#[ derive( Debug, Identifiable, Queryable ) ]
#[ diesel( table_name = articles ) ]
pub struct MArticle
{
  pub id : i32,
  pub title : String,
  pub body : String,
}

#[ derive( Debug, Identifiable, Queryable ) ]
#[ diesel( belongs_to( Article ) ) ]
#[ diesel( table_name = labels ) ]
pub struct MLabel
{
  pub id : i32,
  pub label : String,
  pub article_id : i32,
}

#[ derive( Debug, Insertable ) ]
#[ diesel( table_name = articles ) ]
pub struct NewMArticle
{
  pub title : String,
  pub body : String,
}

#[ derive( Debug, Insertable ) ]
#[ diesel( table_name = labels ) ]
pub struct NewMLabel
{
  pub label : String,
  pub article_id : i32,
}