use config::Config;
use url::Url;
use std::{ time::Duration, net::IpAddr, borrow::Cow };
use serde::{ Serialize, Deserialize };


#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Settings
{
    mode : Mode,
    server : Server,
    db : Db,
    log : Log,
    background : Background,
}

impl Settings
{
  pub fn try_new< CP, EP >( config_path : CP, env_prefix : EP ) -> Result< Self, config::ConfigError >
  where
    CP : Into< Cow< 'static, str > >, EP : Into< Cow< 'static, str > >
  {
     Config::builder()
    .set_default( "mode.debug", false )?

    .set_default( "server.external_url", "http://127.0.0.1" )?
    .set_default( "server.http_port", "8081" )?
    .set_default( "server.grpc_port", "8082" )?
    .set_default( "server.healthz_port", "10025" )?
    .set_default( "server.metrics_port", "9199" )?

    .set_default( "db.mysql.host", "127.0.0.1" )?
    .set_default( "db.mysql.port", "3306" )?
    .set_default( "db.mysql.dating", "default" )?
    .set_default( "db.mysql.user", "root" )?
    .set_default( "db.mysql.pass", "" )?

    .set_default( "db.mysql.connections.max_idle", 30 )?
    .set_default( "db.mysql.connections.max_open", 30 )?

    .set_default( "log.app.level", "info" )?

    .set_default( "background.watchdog.period", "5s" )?
    .set_default( "background.watchdog.limit", 10 )?
    .set_default( "background.watchdog.lock_timeout", "4s" )?

    .add_source( config::File::with_name( &config_path.into() ) )
    .add_source( config::Environment::with_prefix( &env_prefix.into() ) )
    .build()?
    .try_deserialize()
  }
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Mode
{
    debug : bool,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Server
{
    external_url : Url,
    http_port : u16,
    grpc_port : u16,
    healthz_port : u16,
    metrics_port : u16,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Db
{
    mysql : MySql,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct MySql
{
    #[ serde( with = "serde_strz" ) ]
    host : IpAddr,
    port : u16,
    dating : String,
    user : String,
    pass : String,
    connections : DbConnections,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct DbConnections
{
    max_idle : u32,
    max_open : u32,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Log
{
    app : LogApp,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct LogApp
{
    level : LogLevel,
}

#[ derive( Debug, Serialize, Deserialize ) ]
#[ serde( rename_all = "lowercase" ) ]
pub enum LogLevel
{
    Error, Warn, Info, Debug, Trace
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Background
{
    watchdog : Watchdog,
}

#[ derive( Debug, Serialize, Deserialize ) ]
pub struct Watchdog
{
    #[ serde( with = "humantime_serde" ) ]
    period : Duration,
    limit : u32,
    #[ serde( with = "humantime_serde" ) ]
    lock_timeout : Duration,
}