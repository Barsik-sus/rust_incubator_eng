use diesel::prelude::*;
use super::{ schema::*, ID };

#[ derive( Debug, Identifiable, Queryable ) ]
#[ diesel( table_name = users ) ]
pub struct MUser
{
  pub id : ID,
  pub name : String,
}

#[ derive( Debug, Identifiable, Queryable ) ]
#[ diesel( belongs_to( MUser ) ) ]
#[ diesel( table_name = friendship ) ]
pub struct MFriendship
{
  pub id : ID,
  pub user_id : ID,
  pub friend_id : ID,
}

#[ derive( Debug, Insertable ) ]
#[ diesel( table_name = users ) ]
pub struct NewMUser
{
  pub name : String,
}

#[ derive( Debug, Insertable ) ]
#[ diesel( table_name = friendship ) ]
pub struct NewMFriendship
{
  pub user_id : ID,
  pub friend_id : ID,
}

#[ derive( Debug, Queryable, Insertable ) ]
#[ diesel( belongs_to( MUser ) ) ]
#[ diesel( table_name = user_password ) ]
pub struct UserPassword
{
  pub user_id : ID,
  pub password : String
}
