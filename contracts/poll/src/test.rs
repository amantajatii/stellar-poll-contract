#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

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
fn counts_yes_and_no_votes() {
    let env = Env::default();
    let contract_id = env.register(PollContract, ());
    let client = PollContractClient::new(&env, &contract_id);

    client.set_question(&String::from_str(&env, "Do you like Soroban?"));

    assert_eq!(client.vote_yes(), 1);
    assert_eq!(client.vote_yes(), 2);
    assert_eq!(client.vote_no(), 1);

    let result = client.get_result();
    assert_eq!(
        result.question,
        String::from_str(&env, "Do you like Soroban?")
    );
    assert_eq!(result.yes_votes, 2);
    assert_eq!(result.no_votes, 1);
    assert_eq!(result.total_votes, 3);
    assert_eq!(client.total_votes(), 3);
}

#[test]
fn resets_votes_without_changing_question() {
    let env = Env::default();
    let contract_id = env.register(PollContract, ());
    let client = PollContractClient::new(&env, &contract_id);

    client.set_question(&String::from_str(&env, "Reset test?"));
    client.vote_yes();
    client.vote_no();

    assert!(client.reset_votes());

    let result = client.get_result();
    assert_eq!(result.question, String::from_str(&env, "Reset test?"));
    assert_eq!(result.yes_votes, 0);
    assert_eq!(result.no_votes, 0);
    assert_eq!(result.total_votes, 0);
}
