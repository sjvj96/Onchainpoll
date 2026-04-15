#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Data structure for a poll option
#[contracttype]
#[derive(Clone, Debug)]
pub struct PollOption {
    id: u32,
    label: String,
    vote_count: u64,
}

// Data structure for the poll
#[contracttype]
#[derive(Clone, Debug)]
pub struct Poll {
    question: String,
    options: Vec<PollOption>,
    is_active: bool,
}

// Data structure to track voters
#[contracttype]
#[derive(Clone, Debug)]
pub struct VoterRecord {
    voter: String,
    option_id: u32,
}

const POLL_DATA: Symbol = symbol_short!("POLL_DATA");
const VOTER_DATA: Symbol = symbol_short!("VOTERDAT");

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    // Create a new poll with a question and list of option labels
    pub fn create_poll(env: Env, question: String, option_a: String, option_b: String, option_c: String) -> String {
        let options = Vec::from_array(
            &env,
            [
                PollOption { id: 0, label: option_a, vote_count: 0 },
                PollOption { id: 1, label: option_b, vote_count: 0 },
                PollOption { id: 2, label: option_c, vote_count: 0 },
            ],
        );

        let poll = Poll {
            question,
            options,
            is_active: true,
        };

        env.storage().instance().set(&POLL_DATA, &poll);

        // Reset voter records
        let empty_voters: Vec<VoterRecord> = Vec::new(&env);
        env.storage().instance().set(&VOTER_DATA, &empty_voters);

        String::from_str(&env, "Poll created successfully")
    }

    // Cast a vote — each voter address can only vote once
    pub fn vote(env: Env, voter_address: String, option_id: u32) -> String {
        // Check poll exists and is active
        let mut poll: Poll = match env.storage().instance().get(&POLL_DATA) {
            Some(p) => p,
            None => return String::from_str(&env, "No active poll found"),
        };

        if !poll.is_active {
            return String::from_str(&env, "Poll is no longer active");
        }

        // Check if voter already voted
        let mut voters: Vec<VoterRecord> = env
            .storage()
            .instance()
            .get(&VOTER_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..voters.len() {
            if voters.get(i).unwrap().voter == voter_address {
                return String::from_str(&env, "You have already voted");
            }
        }

        // Validate option_id
        if option_id > 2 {
            return String::from_str(&env, "Invalid option ID");
        }

        // Increment vote count on the chosen option
        let mut new_options: Vec<PollOption> = Vec::new(&env);
        for i in 0..poll.options.len() {
            let mut opt = poll.options.get(i).unwrap();
            if opt.id == option_id {
                opt.vote_count += 1;
            }
            new_options.push_back(opt);
        }
        poll.options = new_options;
        env.storage().instance().set(&POLL_DATA, &poll);

        // Record voter
        voters.push_back(VoterRecord {
            voter: voter_address,
            option_id,
        });
        env.storage().instance().set(&VOTER_DATA, &voters);

        String::from_str(&env, "Vote recorded successfully")
    }

    // Get the current poll and results
    pub fn get_results(env: Env) -> Poll {
        env.storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Poll {
                question: String::from_str(&env, "No poll created yet"),
                options: Vec::new(&env),
                is_active: false,
            })
    }

    // Close the poll (no more votes accepted)
    pub fn close_poll(env: Env) -> String {
        let mut poll: Poll = match env.storage().instance().get(&POLL_DATA) {
            Some(p) => p,
            None => return String::from_str(&env, "No poll found"),
        };

        poll.is_active = false;
        env.storage().instance().set(&POLL_DATA, &poll);

        String::from_str(&env, "Poll closed successfully")
    }

    // Get list of all voters and their chosen option
    pub fn get_voters(env: Env) -> Vec<VoterRecord> {
        env.storage()
            .instance()
            .get(&VOTER_DATA)
            .unwrap_or(Vec::new(&env))
    }
}

mod test;