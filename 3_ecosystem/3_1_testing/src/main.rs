use std::
{
  cmp::Ordering,
  env,
  io
};

fn main()
{
  println!( "Guess the number!" );

  let secret_number = get_secret_number();

  loop
  {
    println!( "Please input your guess." );

    let guess = match get_guess_number()
    {
      Some( n ) => n,
      _ => continue,
    };

    println!( "You guessed: {}", guess );

    match guess.cmp( &secret_number )
    {
      Ordering::Less => println!( "Too small!" ),
      Ordering::Greater => println!( "Too big!" ),
      Ordering::Equal =>
      {
        println!( "You win!" );
        break;
      }
    }
  }
}

fn get_secret_number() -> u32
{
  let secret_number = env::args().skip( 1 ).take( 1 ).last().expect( "No secret number is specified" );
  secret_number.trim().parse().ok().expect( "Secret number is not a number" )
}

fn get_guess_number() -> Option< u32 >
{
  let mut guess = String::new();
  io::stdin().read_line( &mut guess ).expect( "Failed to read line" );
  guess.trim().parse().ok()
}

#[ cfg( test ) ]
mod tests
{
  use assert_cmd::Command;
  use proptest::prelude::*;
  
  fn experiment< S >( secret : S, input : S, success : bool, expected_output : S )
  where
    S : Into< String >
  {
    let mut cmd = Command::cargo_bin( "step_3_1" )
    .expect( "Please, build the application first" );
    let assert = cmd
    .arg( secret.into() )
    .write_stdin( input.into() )
    .assert();

    if success
    {
      assert
      .success()
      .stdout( expected_output.into() );
    }
    else
    {
      assert
      .failure()
      .stdout( expected_output.into() );
    }
  }

  #[ test ]
  fn basic()
  {
    experiment
    (
      "25", "0\n50\n25", true, 
r#"Guess the number!
Please input your guess.
You guessed: 0
Too small!
Please input your guess.
You guessed: 50
Too big!
Please input your guess.
You guessed: 25
You win!
"#
    );
  }

  #[ test ]
  fn negative_secret()
  {
    experiment
    (
      "-1", "", false, 
r#"Incorrect secret number!
Secret number must be not-negative integer
"#
    )
  }

  #[ test ]
  fn too_big_secret()
  {
    let too_big_number : u64 = u32::MAX as u64 + 1;
    experiment
    (
      too_big_number.to_string(), "".into(), false, 
r#"Incorrect secret number!
Secret number must be not-negative integer
"#.into() // ? And less then '4294967296' ?
    )
  }

  #[ test ]
  fn negative_input_number()
  {
    experiment
    (
      "3", "-3\n3", true, 
r#"Guess the number!
Please input your guess.
You guessed: -3
Too small!
Please input your guess.
You guessed: 3
You win!
"#
    )
  }
  
  proptest!
  {
    #[ test ]
    fn random_test( n in proptest::num::u32::ANY )
    {
      experiment
      (
        n.to_string(),
        n.to_string(),
        true,
        format!
        (
r#"Guess the number!
Please input your guess.
You guessed: {n}
You win!
"#
        )
      )
    }
  }
}