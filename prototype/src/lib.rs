#![no_std]

use soroban_sdk::{contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env};

// Deposit/withdraw proof only. No yield, transferable shares, adapters or operators.
#[contract]
pub struct DepositProof;

#[contracttype]
#[derive(Clone)]
enum Key {
    Asset,
    Balance(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NonPositiveAmount = 1,
    InsufficientDeposit = 2,
    Overflow = 3,
}

#[contractevent]
pub struct Deposited {
    #[topic]
    pub owner: Address,
    pub amount: i128,
}

#[contractevent]
pub struct Withdrawn {
    #[topic]
    pub owner: Address,
    pub amount: i128,
}

#[contractimpl]
impl DepositProof {
    pub fn __constructor(env: Env, asset: Address) {
        env.storage().instance().set(&Key::Asset, &asset);
    }

    pub fn asset(env: Env) -> Address {
        env.storage().instance().get(&Key::Asset).unwrap()
    }

    pub fn balance(env: Env, owner: Address) -> i128 {
        env.storage().persistent().get(&Key::Balance(owner)).unwrap_or(0)
    }

    pub fn deposit(env: Env, owner: Address, amount: i128) -> Result<i128, Error> {
        owner.require_auth();
        if amount <= 0 { return Err(Error::NonPositiveAmount); }
        let updated = Self::balance(env.clone(), owner.clone())
            .checked_add(amount).ok_or(Error::Overflow)?;
        let asset = Self::asset(env.clone());
        token::Client::new(&env, &asset).transfer(&owner, &env.current_contract_address(), &amount);
        env.storage().persistent().set(&Key::Balance(owner.clone()), &updated);
        maintain_storage(&env, &owner);
        Deposited { owner, amount }.publish(&env);
        Ok(updated)
    }

    pub fn withdraw(env: Env, owner: Address, amount: i128) -> Result<i128, Error> {
        owner.require_auth();
        if amount <= 0 { return Err(Error::NonPositiveAmount); }
        let deposited = Self::balance(env.clone(), owner.clone());
        if amount > deposited { return Err(Error::InsufficientDeposit); }
        let remaining = deposited - amount;
        let asset = Self::asset(env.clone());
        // Both the transfer and balance update commit atomically or revert together.
        env.storage().persistent().set(&Key::Balance(owner.clone()), &remaining);
        token::Client::new(&env, &asset).transfer(&env.current_contract_address(), &owner, &amount);
        maintain_storage(&env, &owner);
        Withdrawn { owner, amount }.publish(&env);
        Ok(remaining)
    }
}

fn maintain_storage(env: &Env, owner: &Address) {
    env.storage().instance().extend_ttl(17_280, 120_960);
    env.storage().persistent().extend_ttl(&Key::Balance(owner.clone()), 17_280, 120_960);
}

#[cfg(test)]
mod test;
