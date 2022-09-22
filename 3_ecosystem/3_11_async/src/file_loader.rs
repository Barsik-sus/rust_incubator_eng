use std::{ borrow::Cow, io::Write };

#[ derive( Debug ) ]
pub struct LoadFile
{
  client : reqwest::Client,
  url : Cow< 'static, str >,
  file : std::fs::File,
  pub file_path : String,
}

impl LoadFile
{
  pub fn new< S >( client : reqwest::Client, url : S, file_path : S ) -> Result< Self, String >
  where
    S : Into< Cow< 'static, str > >
  {
    let path : String = file_path.into().to_string();
    let file = std::fs::File::create( &path )
    .or( Err( format!( "Failed to create file '{}'", &path ) ) )?;

    Ok( Self
    {
      client,
      url : url.into(),
      file,
      file_path : path,
    })
  }

  pub async fn load( mut self ) -> Result< (), String >
  {
    let response = self.client
    .get( &*self.url )
    .send()
    .await.or( Err( format!( "Can not GET '{}'", &self.url ) ) )?;

    let bytes = response
    .bytes()
    .await.or( Err( format!( "Can not take bytes from '{}'", &self.url ) ) )?;

    self.file.write_all( &bytes )
    .or( Err( format!( "Can not write to file '{}'", &self.file_path ) ) )?;

    Ok( () )
  }
}