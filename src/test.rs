#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal
};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    assert_eq!(client.owner(), owner);
    assert_eq!(client.token_address(), token_address);
    assert_eq!(client.total_supply(), 0);
}

#[test]
fn test_deposit_and_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    let amount = 1000i128;
    client.deposit(&user, &amount);
    
    assert_eq!(client.balance(&user), amount);
    assert_eq!(client.total_supply(), amount);
}

#[test]
fn test_withdraw() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    let deposit_amount = 1000i128;
    let withdraw_amount = 400i128;
    
    client.deposit(&user, &deposit_amount);
    client.withdraw(&user, &withdraw_amount);
    
    assert_eq!(client.balance(&user), deposit_amount - withdraw_amount);
    assert_eq!(client.total_supply(), deposit_amount - withdraw_amount);
}

#[test]
#[should_panic(expected = "InsufficientBalance")]
fn test_withdraw_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    let deposit_amount = 500i128;
    let withdraw_amount = 1000i128;
    
    client.deposit(&user, &deposit_amount);
    client.withdraw(&user, &withdraw_amount);
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_deposit_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    client.deposit(&user, &0i128);
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_withdraw_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, Vault);
    let client = VaultClient::new(&env, &contract_id);
    
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let token_address = Address::generate(&env);
    
    client.initialize(&owner, &token_address);
    
    client.deposit(&user, &1000i128);
    client.withdraw(&user, &-100i128);
}