use step_2_1::Post;

fn main()
{
  let post = Post::new( "Test".into(), "Text".into() );
  post.allow();
}