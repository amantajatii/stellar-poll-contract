#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn sets_and_reads_question() {
    let env = Env::default();
    let contract_id = env.register(PollContract, ());
    let client = PollContractClient::new(&env, &contract_id);

    assert_eq!(
        client.get_question(),
        String::from_str(&env, "No question set")
    );

    assert!(client.set_question(&String::from_str(
        &env,
        "Should Stellar be used for workshop apps?"
    )));
    assert_eq!(
        client.get_question(),
        String::from_str(&env, "Should Stellar be used for workshop apps?")
    );
}

#[test]
fn allows_one_vote_per_wallet_per_round() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PollContract, ());
    let client = PollContractClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.set_question(&String::from_str(&env, "Do you like Soroban?"));

    assert!(client.vote_yes(&alice));
    assert!(!client.vote_yes(&alice));
    assert!(!client.vote_no(&alice));
    assert!(client.vote_no(&bob));

    let result = client.get_result();
    assert_eq!(result.yes_votes, 1);
    assert_eq!(result.no_votes, 1);
    assert_eq!(result.total_votes, 2);
    assert_eq!(result.round, 1);
    assert!(client.has_voted(&alice));
    assert_eq!(client.get_vote(&alice), symbol_short!("YES"));
}

#[test]
fn reset_starts_new_round() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PollContract, ());
    let client = PollContractClient::new(&env, &contract_id);
    let alice = Address::generate(&env);

    client.set_question(&String::from_str(&env, "Reset test?"));
    assert!(client.vote_yes(&alice));
    assert!(!client.vote_no(&alice));

    assert!(client.reset_votes());
    assert_eq!(client.round(), 2);
    assert!(!client.has_voted(&alice));
    assert!(client.vote_no(&alice));

    let result = client.get_result();
    assert_eq!(result.question, String::from_str(&env, "Reset test?"));
    assert_eq!(result.yes_votes, 0);
    assert_eq!(result.no_votes, 1);
    assert_eq!(result.total_votes, 1);
    assert_eq!(result.round, 2);
}
