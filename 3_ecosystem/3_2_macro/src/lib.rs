use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, Expr, parse::Parse, punctuated::Punctuated, Token };

struct Args
{
  vars : Vec< Expr >
}

impl Parse for Args
{
  fn parse( input : syn::parse::ParseStream ) -> syn::Result< Self >
  {
    let vars = Punctuated::< Expr, Token![ , ] >::parse_terminated( input )?;
    Ok
    (
      Args { vars : vars.into_iter().collect() }
    )
  }
}

#[ proc_macro ]
pub fn pbtreemap( input : TokenStream ) -> TokenStream
{
  let input = parse_macro_input!( input as Args ).vars;
  TokenStream::from( quote!
  {{
    let mut tmp_btreemap = BTreeMap::new();

    for ( key, value ) in [ #(#input),* ]
    {
      tmp_btreemap.insert( key, value );
    }

    tmp_btreemap
  }})
}
