#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror, 
    Address, Env, token
};

mod test;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance(Address),
    Owner,
    TokenAddress,
    TotalSupply,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    NotAuthorized = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    NotInitialized = 4,
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    pub fn initialize(env: Env, owner: Address, token_address: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Contract already initialized");
        }
        
        owner.require_auth();
        
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::TokenAddress, &token_address);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();
        
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .ok_or(VaultError::NotInitialized)?;

        let token_client = token::Client::new(&env, &token_address);
        
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        let current_balance = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0i128);
        
        let new_balance = current_balance + amount;
        env.storage().persistent().set(&DataKey::Balance(user), &new_balance);

        let total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply + amount));

        Ok(())
    }

    pub fn withdraw(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();
        
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let current_balance = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0i128);

        if current_balance < amount {
            return Err(VaultError::InsufficientBalance);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .ok_or(VaultError::NotInitialized)?;

        let token_client = token::Client::new(&env, &token_address);
        
        token_client.transfer(&env.current_contract_address(), &user, &amount);

        let new_balance = current_balance - amount;
        env.storage().persistent().set(&DataKey::Balance(user), &new_balance);

        let total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply - amount));

        Ok(())
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn owner(env: Env) -> Result<Address, VaultError> {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(VaultError::NotInitialized)
    }

    pub fn token_address(env: Env) -> Result<Address, VaultError> {
        env.storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .ok_or(VaultError::NotInitialized)
    }
}