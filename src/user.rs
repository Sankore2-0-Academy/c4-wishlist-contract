use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen};

use crate::vehicle::Vehicle;

/**
 * User structure
 */
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
 wishlist: Vec<Vehicle>,
}

impl Default for User {
  fn default() -> Self {
    Self {
      wishlist: vec![]
    }
  }
}

#[near_bindgen]
impl User {
  // Initializes a new user object
 pub fn new_user() -> Self {
  Self {
   wishlist: vec![]
  }
 }

 /**
  * Adds a car object to the vehicles Vec
  */
 pub fn add(&mut self, image: String, name: String, model: String, mileage: u64, year: String, price: f64) {
  let vehicle: Vehicle = Vehicle::new(image, name, model, mileage, year, price);
  self.wishlist.push(vehicle);
 }

 /**
  * Retrieves car objects from the vehicles Vec
  */
 pub fn show(&self, start: u32, limit: u32) -> Vec<Vehicle> {
  let result: Vec<Vehicle> = self.wishlist.iter().skip(start as usize).take(limit as usize).cloned().collect();
  result
 }

 /**
  * Deletes a car object from the vehicles Vec
  */
 pub fn remove(&mut self, index: u64) -> Vehicle {
  let size: u64 = self.wishlist.len() as u64;
  assert!(size > 0 && index < size, "Invalid car id!");
  self.wishlist.remove(index as usize)
 }
}
