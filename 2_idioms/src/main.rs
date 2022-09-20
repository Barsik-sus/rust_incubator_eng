use std::{ borrow::Cow, collections::HashMap, cmp::Reverse };
use strum::{ IntoEnumIterator, EnumIter };

#[ derive( Debug, Clone, Copy, PartialEq, EnumIter ) ]
pub enum Coin
{
  One = 1,
  Two = 2,
  Five = 5,
  Ten = 10,
  Twenty = 20,
  Fifty = 50,
}

#[ derive( Debug, PartialEq ) ]
pub struct Money( Vec< Coin > );

impl From< i32 > for Money
{
    fn from( mut amount : i32 ) -> Self
    {
      let mut result = vec![];
      let mut variants = Coin::iter().collect::< Vec< _ > >();

      variants.sort_by_key( | &coin | Reverse( coin as i32 ) );
      for coin in variants
      {
        while amount - coin as i32 >= 0
        {
          amount -= coin as i32;
          result.push( coin );
        }
      }

      Money( result )
    }
}

#[ derive( Debug, Clone, PartialEq ) ]
pub struct Product
{
  pub name : Cow< 'static, str >,
  pub price : i32,
}

impl Product
{
  pub fn new< S >( name : S, price : i32 ) -> Self
  where
    S : Into< Cow< 'static, str > >
  {
    Self { name: name.into(), price }
  }
}

#[ derive( Debug, PartialEq ) ]
pub enum OrderError
{
  NotEnoughMoney,
  ProductNotFound
}

#[ derive( Debug ) ]
pub struct VendingMachine
{
  products : HashMap< Cow< 'static, str >, Product >,
  money_order : i32,
  _capacity : usize,
}

impl VendingMachine
{
  pub fn add_coin( &mut self, coin : Coin )
  {
    self.money_order += coin as i32
  }

  pub fn try_to_buy< S >( &mut self, product_name : S ) -> Result< ( Product, Money ), OrderError >
  where
    S : Into< Cow< 'static, str > >
  {
    let product = match self.products.get( &product_name.into() )
    {
      None => Err( OrderError::ProductNotFound ),
      Some( product ) =>
      {
        if product.price > self.money_order
        {
          Err( OrderError::NotEnoughMoney )
        }
        else
        {
          Ok( ( product.name.clone(), product.price ) )
        }
      }
    }?;
    let change = self.money_order - product.1;
    self.money_order = 0;
    Ok( ( self.products.remove( &product.0 ).unwrap(), change.into() ) )
  }

  pub fn get_money_back( &mut self ) -> Money
  {
    std::mem::take( &mut self.money_order ).into()
  }
}

#[ derive( Debug ) ]
pub struct VendingMachineOverflow;

#[ derive( Debug ) ]
pub struct VendingMachineBuilder
{
  products : Vec< Product >,
  capacity : usize,
}

impl VendingMachineBuilder
{
  pub fn new( capacity : usize ) -> Self
  {
    Self { products: vec![], capacity }
  }

  pub fn try_add_product( mut self, product : Product ) -> Result< Self, VendingMachineOverflow >
  {
    if self.capacity > self.products.len()
    {
      self.products.push( product );
      Ok( self )
    }
    else
    {
      Err( VendingMachineOverflow )
    }
  }

  pub fn build( self ) -> VendingMachine
  {
    VendingMachine
    {
      products : self.products.iter().cloned()
      .map( | product | ( product.name.clone(), product ) )
      .collect(),
      money_order : 0,
      _capacity : self.capacity,
    }
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn basic()
  {
    let mut vm = VendingMachineBuilder::new( 3 )
    .try_add_product( Product::new( "Cat", 97 ) ).unwrap()
    .try_add_product( Product::new( "Candy", 50 ) ).unwrap()
    .build();

    assert_eq!( Err( OrderError::NotEnoughMoney ), vm.try_to_buy( "Cat" ) );
    assert_eq!( Err( OrderError::ProductNotFound ), vm.try_to_buy( "Dog" ) );

    assert_eq!( Money( vec![] ), vm.get_money_back() );
    assert_eq!( 0, vm.money_order );

    vm.add_coin( Coin::Fifty );
    vm.add_coin( Coin::Fifty );

    assert_eq!( Money( vec![ Coin::Fifty, Coin::Fifty ] ), vm.get_money_back() );
    assert_eq!( 0, vm.money_order );

    vm.add_coin( Coin::Fifty );
    vm.add_coin( Coin::Twenty );
    vm.add_coin( Coin::Twenty );
    vm.add_coin( Coin::Twenty );

    assert_eq!
    (
      Ok
      ((
          Product{ name : "Cat".into(), price : 97 },
          Money( vec![ Coin::Ten, Coin::Two, Coin::One ] )
      )),
      vm.try_to_buy( "Cat" )
    );
  }
}