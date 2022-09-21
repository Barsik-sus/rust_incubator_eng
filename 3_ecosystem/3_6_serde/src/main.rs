use std::time::Duration;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use url::Url;
use uuid::Uuid;


#[ derive( Debug, Serialize, Deserialize ) ]
#[ serde( tag = "type" ) ]
enum RequestType
{
  #[ serde( rename = "success" ) ]
  Success
  {
    stream : Stream,
    gifts : Vec< Gift >
  },
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct Request
{
  #[ serde( rename = "type" ) ]
  #[ serde( flatten ) ]
  rtype : RequestType,
  debug : DebugInfo,
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct Stream
{
  user_id : Uuid,
  is_private : bool,
  settings : u32,
  shard_url : Url,
  public_tariff : PublicTariff,
  private_tariff : PrivateTariff,
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct PublicTariff
{
  id : u32,
  price : f64,
  #[ serde( with = "humantime_serde" ) ]
  duration : Duration,
  description : String,
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct PrivateTariff
{
  client_price : f64,
  #[ serde( with = "humantime_serde" ) ]
  duration : Duration,
  description : String,
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct Gift
{
  id : u32,
  price : f64,
  description : String,
}

#[ derive( Debug, Serialize, Deserialize ) ]
struct DebugInfo
{
  #[ serde( with = "humantime_serde" ) ]
  duration : Duration,
  at : DateTime< Utc >,
}

fn main()
{
  let content =
  {
    match std::fs::read( "./request.json" )
    {
      Err( _ ) => std::fs::read( "./3_ecosystem/3_6_serde/request.json"),
      Ok( data ) => Ok( data ),
    }
  }.unwrap();
  let request = serde_json::from_slice::< Request >( &content ).unwrap();
  std::fs::write( "request.yaml", serde_yaml::to_string( &request ).unwrap() ).unwrap();
  std::fs::write( "request.toml", toml::to_string( &request ).unwrap() ).unwrap();
}
