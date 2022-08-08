#!/bin/bash 

SIGNER=<signer-account-id>

source ./scripts/setting.conf


# Add Car to wishlist
# near call $SUB_ACCOUNT add_car '{"image": "https://www.ccarprice.com/products/Toyota_RAV4_Hybrid_LE_2022.jpg", "name": "Toyota", "model": "RAV4", "mileage": 1000, "year": "2022", "price": 5000000}' --accountId $SIGNER --amount 1

# Show wishlist content
near call $SUB_ACCOUNT read_wishlist '{"start": 0, "limit": 10}' --accountId $SIGNER

# Remove car from wishlist
# near call $SUB_ACCOUNT delete_car '{"id": 0}' --accountId $SIGNER
