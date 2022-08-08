use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen};


#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Vehicle {
 pub image: String,
 pub name: String,
 pub model: String,
 pub mileage: u64,
 pub year: String,
 pub price: f64
}

#[near_bindgen]
impl Vehicle {
 pub fn new(image: String, name: String, model: String, mileage: u64, year: String, price: f64) -> Self {
  Self { 
   image,
   name, 
   model, 
   mileage, 
   year, 
   price 
  }
 }
}