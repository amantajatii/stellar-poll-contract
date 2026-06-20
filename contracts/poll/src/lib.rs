#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PollResult {
    pub question: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub total_votes: u32,
}

const QUESTION: Symbol = symbol_short!("QUESTION");
const YES_VOTES: Symbol = symbol_short!("YES");
const NO_VOTES: Symbol = symbol_short!("NO");

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    pub fn set_question(env: Env, question: String) -> bool {
        env.storage().instance().set(&QUESTION, &question);
        env.storage().instance().extend_ttl(100, 518400);
        true
    }

    pub fn get_question(env: Env) -> String {
        env.storage()
            .instance()
            .get(&QUESTION)
            .unwrap_or(String::from_str(&env, "No question set"))
    }

    pub fn vote_yes(env: Env) -> u32 {
        let votes = Self::yes_votes(env.clone()) + 1;
        env.storage().instance().set(&YES_VOTES, &votes);
        env.storage().instance().extend_ttl(100, 518400);
        votes
    }

    pub fn vote_no(env: Env) -> u32 {
        let votes = Self::no_votes(env.clone()) + 1;
        env.storage().instance().set(&NO_VOTES, &votes);
        env.storage().instance().extend_ttl(100, 518400);
        votes
    }

    pub fn yes_votes(env: Env) -> u32 {
        env.storage().instance().get(&YES_VOTES).unwrap_or(0)
    }

    pub fn no_votes(env: Env) -> u32 {
        env.storage().instance().get(&NO_VOTES).unwrap_or(0)
    }

    pub fn total_votes(env: Env) -> u32 {
        Self::yes_votes(env.clone()) + Self::no_votes(env)
    }

    pub fn get_result(env: Env) -> PollResult {
        let yes_votes = Self::yes_votes(env.clone());
        let no_votes = Self::no_votes(env.clone());

        PollResult {
            question: Self::get_question(env),
            yes_votes,
            no_votes,
            total_votes: yes_votes + no_votes,
        }
    }

    pub fn reset_votes(env: Env) -> bool {
        env.storage().instance().set(&YES_VOTES, &0u32);
        env.storage().instance().set(&NO_VOTES, &0u32);
        env.storage().instance().extend_ttl(100, 518400);
        true
    }
}

mod test;
