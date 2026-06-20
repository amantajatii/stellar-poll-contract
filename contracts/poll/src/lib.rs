#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PollResult {
    pub question: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub total_votes: u32,
    pub round: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Question,
    YesVotes,
    NoVotes,
    Round,
    Vote(Address, u32),
}

const YES: Symbol = symbol_short!("YES");
const NO: Symbol = symbol_short!("NO");

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    pub fn set_question(env: Env, question: String) -> bool {
        env.storage().instance().set(&DataKey::Question, &question);
        env.storage().instance().extend_ttl(100, 518400);
        true
    }

    pub fn get_question(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Question)
            .unwrap_or(String::from_str(&env, "No question set"))
    }

    pub fn vote_yes(env: Env, voter: Address) -> bool {
        Self::vote(env, voter, YES)
    }

    pub fn vote_no(env: Env, voter: Address) -> bool {
        Self::vote(env, voter, NO)
    }

    pub fn has_voted(env: Env, voter: Address) -> bool {
        let key = DataKey::Vote(voter, Self::round(env.clone()));
        env.storage().persistent().has(&key)
    }

    pub fn get_vote(env: Env, voter: Address) -> Symbol {
        let key = DataKey::Vote(voter, Self::round(env.clone()));
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(symbol_short!("NONE"))
    }

    pub fn yes_votes(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::YesVotes)
            .unwrap_or(0)
    }

    pub fn no_votes(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::NoVotes).unwrap_or(0)
    }

    pub fn total_votes(env: Env) -> u32 {
        Self::yes_votes(env.clone()) + Self::no_votes(env)
    }

    pub fn round(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Round).unwrap_or(1)
    }

    pub fn get_result(env: Env) -> PollResult {
        let yes_votes = Self::yes_votes(env.clone());
        let no_votes = Self::no_votes(env.clone());

        PollResult {
            question: Self::get_question(env.clone()),
            yes_votes,
            no_votes,
            total_votes: yes_votes + no_votes,
            round: Self::round(env),
        }
    }

    pub fn reset_votes(env: Env) -> bool {
        let next_round = Self::round(env.clone()) + 1;
        env.storage().instance().set(&DataKey::YesVotes, &0u32);
        env.storage().instance().set(&DataKey::NoVotes, &0u32);
        env.storage().instance().set(&DataKey::Round, &next_round);
        env.storage().instance().extend_ttl(100, 518400);
        true
    }

    fn vote(env: Env, voter: Address, choice: Symbol) -> bool {
        voter.require_auth();

        let round = Self::round(env.clone());
        let key = DataKey::Vote(voter, round);
        if env.storage().persistent().has(&key) {
            return false;
        }

        if choice == YES {
            let votes = Self::yes_votes(env.clone()) + 1;
            env.storage().instance().set(&DataKey::YesVotes, &votes);
        } else {
            let votes = Self::no_votes(env.clone()) + 1;
            env.storage().instance().set(&DataKey::NoVotes, &votes);
        }

        env.storage().persistent().set(&key, &choice);
        env.storage().persistent().extend_ttl(&key, 100, 518400);
        env.storage().instance().extend_ttl(100, 518400);
        true
    }
}

mod test;
