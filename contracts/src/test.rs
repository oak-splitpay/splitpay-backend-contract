#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, Env, String,
};

#[test]
fn test_init_split_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver = Address::generate(&env);
    let split_id = String::from_str(&env, "split-001");
    let total_amount: i128 = 40_000_000; // 40 tokens (e.g. 40 XLM in stroops)
    let num_participants: u32 = 4;

    let per_person = client.init_split(
        &creator,
        &split_id,
        &receiver,
        &None,
        &total_amount,
        &num_participants,
    );

    assert_eq!(per_person, 10_000_000);

    let status = client.get_split_status(&split_id);
    assert_eq!(status.id, split_id);
    assert_eq!(status.receiver, receiver);
    assert_eq!(status.total_amount, total_amount);
    assert_eq!(status.num_participants, num_participants);
    assert_eq!(status.amount_per_person, 10_000_000);
    assert_eq!(status.payments_received, 0);
    assert_eq!(status.is_completed, false);
}

#[test]
fn test_init_split_invalid_params() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver = Address::generate(&env);

    // Invalid amount (0)
    let res1 = client.try_init_split(
        &creator,
        &String::from_str(&env, "s1"),
        &receiver,
        &None,
        &0,
        &4,
    );
    assert_eq!(res1, Err(Ok(SplitpayError::InvalidAmount)));

    // Invalid participants (< 2)
    let res2 = client.try_init_split(
        &creator,
        &String::from_str(&env, "s2"),
        &receiver,
        &None,
        &10_000_000,
        &1,
    );
    assert_eq!(res2, Err(Ok(SplitpayError::InvalidParticipants)));
}

#[test]
fn test_init_split_duplicate_id() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver = Address::generate(&env);
    let split_id = String::from_str(&env, "dup-split");

    client.init_split(
        &creator,
        &split_id,
        &receiver,
        &None,
        &20_000_000,
        &2,
    );

    let res = client.try_init_split(
        &creator,
        &split_id,
        &receiver,
        &None,
        &20_000_000,
        &2,
    );
    assert_eq!(res, Err(Ok(SplitpayError::SplitAlreadyExists)));
}

#[test]
fn test_record_payment_and_completion() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver = Address::generate(&env);
    let split_id = String::from_str(&env, "split-team");

    let per_person = client.init_split(
        &creator,
        &split_id,
        &receiver,
        &None,
        &30_000_000,
        &3,
    );
    assert_eq!(per_person, 10_000_000);

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    // Participant 1 pays
    assert!(!client.has_participant_paid(&split_id, &p1));
    let complete1 = client.record_payment(&split_id, &p1, &10_000_000);
    assert_eq!(complete1, false);
    assert!(client.has_participant_paid(&split_id, &p1));

    // Duplicate payment from p1 should fail
    let dup_pay = client.try_record_payment(&split_id, &p1, &10_000_000);
    assert_eq!(dup_pay, Err(Ok(SplitpayError::ParticipantAlreadyPaid)));

    // Participant 2 pays
    let complete2 = client.record_payment(&split_id, &p2, &10_000_000);
    assert_eq!(complete2, false);

    // Participant 3 pays with wrong amount should fail
    let wrong_amt = client.try_record_payment(&split_id, &p3, &5_000_000);
    assert_eq!(wrong_amt, Err(Ok(SplitpayError::InvalidAmount)));

    // Participant 3 pays correct amount
    let complete3 = client.record_payment(&split_id, &p3, &10_000_000);
    assert_eq!(complete3, true);

    // Status check
    assert!(client.is_split_complete(&split_id));
    let status = client.get_split_status(&split_id);
    assert_eq!(status.payments_received, 3);
    assert_eq!(status.is_completed, true);

    // Trying to pay after completion should fail
    let p4 = Address::generate(&env);
    let after_complete = client.try_record_payment(&split_id, &p4, &10_000_000);
    assert_eq!(after_complete, Err(Ok(SplitpayError::SplitAlreadyCompleted)));
}

#[test]
fn test_record_payment_with_token_settlement() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_admin_client = StellarAssetClient::new(&env, &token_contract.address());
    let token_client = TokenClient::new(&env, &token_contract.address());

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver = Address::generate(&env);
    let split_id = String::from_str(&env, "dinner-split");

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);

    // Mint tokens to participants
    token_admin_client.mint(&p1, &50_000_000);
    token_admin_client.mint(&p2, &50_000_000);

    // Init split for 50 total tokens across 2 participants (25 each)
    client.init_split(
        &creator,
        &split_id,
        &receiver,
        &Some(token_contract.address()),
        &50_000_000,
        &2,
    );

    // Initial receiver balance is 0
    assert_eq!(token_client.balance(&receiver), 0);

    // P1 pays 25
    client.record_payment(&split_id, &p1, &25_000_000);
    assert_eq!(token_client.balance(&p1), 25_000_000);
    assert_eq!(token_client.balance(&receiver), 25_000_000);

    // P2 pays 25
    let completed = client.record_payment(&split_id, &p2, &25_000_000);
    assert_eq!(completed, true);
    assert_eq!(token_client.balance(&p2), 25_000_000);
    assert_eq!(token_client.balance(&receiver), 50_000_000);
}

#[test]
fn test_multiple_independent_splits() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitpayContract, ());
    let client = SplitpayContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let receiver1 = Address::generate(&env);
    let receiver2 = Address::generate(&env);

    let s1 = String::from_str(&env, "session-1");
    let s2 = String::from_str(&env, "session-2");

    client.init_split(&creator, &s1, &receiver1, &None, &20_000_000, &2);
    client.init_split(&creator, &s2, &receiver2, &None, &90_000_000, &3);

    let status1 = client.get_split_status(&s1);
    let status2 = client.get_split_status(&s2);

    assert_eq!(status1.receiver, receiver1);
    assert_eq!(status1.amount_per_person, 10_000_000);

    assert_eq!(status2.receiver, receiver2);
    assert_eq!(status2.amount_per_person, 30_000_000);
}
