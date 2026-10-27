use super::*;
use soroban_sdk::testutils::Address as _;

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin).address();
    env.mock_all_auths();
    token::StellarAssetClient::new(&env, &asset).mint(&user, &1_000_000_000);
    let vault = env.register(DepositProof, (asset.clone(),));
    (env, asset, user, vault)
}

#[test]
fn deposit_and_full_withdraw_return_funds() {
    let (env, asset, user, vault) = setup();
    let v = DepositProofClient::new(&env, &vault);
    let t = token::Client::new(&env, &asset);
    assert_eq!(v.deposit(&user, &100_000_000), 100_000_000);
    assert_eq!(v.balance(&user), 100_000_000);
    assert_eq!(t.balance(&vault), 100_000_000);
    assert_eq!(t.balance(&user), 900_000_000);
    assert_eq!(v.withdraw(&user, &100_000_000), 0);
    assert_eq!(t.balance(&user), 1_000_000_000);
    assert_eq!(t.balance(&vault), 0);
}

#[test]
fn deposits_are_isolated_by_owner() {
    let (env, asset, user, vault) = setup();
    let other = Address::generate(&env);
    token::StellarAssetClient::new(&env, &asset).mint(&other, &200);
    let v = DepositProofClient::new(&env, &vault);
    v.deposit(&user, &100);
    v.deposit(&other, &200);
    assert_eq!(v.withdraw(&other, &50), 150);
    assert_eq!(v.balance(&user), 100);
    assert_eq!(token::Client::new(&env, &asset).balance(&vault), 250);
}

#[test]
fn zero_and_negative_amounts_are_rejected() {
    let (env, _, user, vault) = setup();
    let v = DepositProofClient::new(&env, &vault);
    assert!(v.try_deposit(&user, &0).is_err());
    assert!(v.try_deposit(&user, &-1).is_err());
    assert!(v.try_withdraw(&user, &0).is_err());
    assert!(v.try_withdraw(&user, &-1).is_err());
    assert_eq!(v.balance(&user), 0);
}

#[test]
fn overwithdraw_preserves_accounting() {
    let (env, asset, user, vault) = setup();
    let v = DepositProofClient::new(&env, &vault);
    v.deposit(&user, &100);
    assert!(v.try_withdraw(&user, &101).is_err());
    assert_eq!(v.balance(&user), 100);
    assert_eq!(token::Client::new(&env, &asset).balance(&vault), 100);
}

#[test]
fn failed_token_transfer_does_not_credit_deposit() {
    let (env, _, user, vault) = setup();
    let v = DepositProofClient::new(&env, &vault);
    assert!(v.try_deposit(&user, &1_000_000_001).is_err());
    assert_eq!(v.balance(&user), 0);
}

#[test]
fn unauthenticated_actions_are_rejected() {
    let (env, _, user, vault) = setup();
    let v = DepositProofClient::new(&env, &vault);
    v.deposit(&user, &100);
    env.mock_auths(&[]);
    assert!(v.try_deposit(&user, &10).is_err());
    assert!(v.try_withdraw(&user, &10).is_err());
    assert_eq!(v.balance(&user), 100);
}
