/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, env, AccountId, Promise};

mod user;
mod vehicle;

use user::User;
use vehicle::Vehicle;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Wishlist {
    users: LookupMap<AccountId, User>,
}

impl Default for Wishlist {
  fn default() -> Self {
    Self {
      users: LookupMap::new(b"c"),
    }
  }
}

#[near_bindgen]
impl Wishlist {
    /**
     * Adds a new car object to user's wishlist
     */
    #[payable]
    pub fn add_car(&mut self, image: String, name: String, model: String, mileage: u64, year: String, price: u64) {
        // Get user account id
        let signer = env::predecessor_account_id();

        // get attached deposit
        let deposit = env::attached_deposit();

        // Get initial storage space used
        let initial_storage = env::storage_usage();

        // Check if the user already exists
        if let Some(mut user) = self.users.get(&signer) {
            // Update user object with the car info
            user.add(
                image,
                name, 
                model, 
                mileage, 
                year, 
                price as f64
            );
            // Update user object on blockchain
            self.users.insert(&signer, &user);

            // Settle storage cost
            self.pay_for_storage(initial_storage, deposit);
        } else {
            // Initialize a new user object
            let mut user = User::new_user();

            // Update user object with the car info
            user.add(
                image,
                name, 
                model, 
                mileage, 
                year, 
                price as f64
            );

            // Persist user object on blockchain
            self.users.insert(&signer, &user);

            // Settle storage cost
            self.pay_for_storage(initial_storage, deposit);
        }
    }

    /**
     * Retreives a paginated user car wishlist
     */
    pub fn read_wishlist(&self, start: u32, limit: u32) -> Option<Vec<Vehicle>> {
        // Get user account id
        let signer = env::predecessor_account_id();

        // Check if user record exist in users storage
        if let Some(user) = self.users.get(&signer) {
            // Get a list of car objects in user wishlist
            let vehicles: Vec<Vehicle> = user.show(start, limit);
            // Return the list
            Some(vehicles)
        } else {
            // Return empty list
            Some(vec![])
        }
    }

    /**
     * Remove a car object from the user's wishlist given its id (index)
     */
    pub fn delete_car(&mut self, id: u64) -> Option<Vehicle> {
        // Get user account id
        let signer = env::predecessor_account_id();

        // Get initial storage space occupied
        let initial_storage = env::storage_usage();

        // Check if user record exist in users storage
        if let Some(mut user) = self.users.get(&signer) {
            // Delete the car object from user wishlist
            let removed_vehicle = user.remove(id);

            // Update user object on blockchain
            self.users.insert(&signer, &user);

            // Credit the tokens unlocked after releasing storage space
            self.refund_storage_cost(initial_storage);

            // Return deleted car object
            Some(removed_vehicle)
        } else {
            // Return Null
            None
        }
    }


    /**
     * Settles storage expenses
     */
    fn pay_for_storage(&self, initial_storage: u64, attached_storage_cost: u128) {
        // Get Current Storage
        let current_storage = env::storage_usage();
        
        // Get Storage Used
        let storage_used = current_storage - initial_storage;
        
        // Get Storage cost per byte
        let storage_cost: u128 = env::storage_byte_cost();
        
        // Get payable storage fee
        if let Some(total_storage_cost) = storage_cost.checked_mul(storage_used as u128) {
            // Check if user attached enough tokens to cater for storage
            assert!(attached_storage_cost >= total_storage_cost, "Insufficient funds!");
            
            // Check for balance
            let excess_balance = attached_storage_cost - total_storage_cost;
            if excess_balance > 0 {
                // Return excess tokens to user
                self.return_excess_tokens(excess_balance);
            }
        }
    }
    
    /**
    * Sends back excess tokens to user
    */
    pub fn return_excess_tokens(&self, excess_balance: u128) {
        // Get signer address
        let signer = env::predecessor_account_id();
        
        // Send back excess
        Promise::new(signer).transfer(excess_balance);
    }

    /**
     * Refunds user on storage release
     */
    fn refund_storage_cost(&self, initial_storage: u64) {
        // Get current storage space
        let current_storage = env::storage_usage();

        // Compute storage space released
        let storage_released = initial_storage - current_storage;

        // Get storage unit price (per byte)
        let storage_unit_price = env::storage_byte_cost();

        // Compute total refundable storage cost
        if let Some(refundable_storage_cost) = storage_unit_price.checked_mul(storage_released.into()) {
            // Transfer to user wallet address
            self.return_excess_tokens(refundable_storage_cost);
        } else {
            panic!("Error calculating storage cost");
        }
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;    
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::


    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .attached_deposit(1000000000000000000000000)
            .build()
    }
    
    fn get_params() -> (String, String, String, u64, String, u64) {
        let image: String = String::from("https://www.ccarprice.com/products/Toyota_RAV4_Hybrid_LE_2022.jpg");
        let name: String = String::from("Toyota");
        let model: String = String::from("RAV4");
        let mileage: u64 = 10000;
        let year: String = String::from("2022");
        let price: u64 = 10000000;
        (image, name, model, mileage, year, price)
    }

    #[test]
    fn add_to_wishlist() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Wishlist::default();
        let params = get_params();

        contract.add_car(params.0, params.1, params.2, params.3, params.4, params.5);

        if let Some(vehicles) = contract.read_wishlist(0, 3) {
            assert_eq!(1, vehicles.len());
            let test_params = get_params();
            assert_eq!(&vehicles[0].model, &test_params.2);
        } else {
            panic!("Error in the code");
        }
        
    }

    #[test]
    fn remove_from_wishlist() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Wishlist::default();
        let params = get_params();
        contract.add_car(params.0, params.1, params.2, params.3, params.4, params.5);

        if let Some(vehicles) = contract.read_wishlist(0, 3) {
            assert_eq!(1, vehicles.len());
        } else {
            panic!("Error reading wishlist");
        }

        // Remove functionality
        contract.delete_car(0);

        if let Some(vehicles) = contract.read_wishlist(0, 3) {
            assert_eq!(0, vehicles.len());
        } else {
            panic!("Error reading wishlist");
        }
    }
}

