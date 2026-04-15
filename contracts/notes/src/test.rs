#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_create_and_vote() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PollContract);
        let client = PollContractClient::new(&env, &contract_id);

        // Create poll
        let result = client.create_poll(
            &String::from_str(&env, "What is your favorite blockchain?"),
            &String::from_str(&env, "Stellar"),
            &String::from_str(&env, "Ethereum"),
            &String::from_str(&env, "Solana"),
        );
        assert_eq!(result, String::from_str(&env, "Poll created successfully"));

        // Vote
        let vote_result = client.vote(
            &String::from_str(&env, "GDEMO_WALLET_1"),
            &0,
        );
        assert_eq!(vote_result, String::from_str(&env, "Vote recorded successfully"));

        // Duplicate vote should be rejected
        let dup = client.vote(
            &String::from_str(&env, "GDEMO_WALLET_1"),
            &1,
        );
        assert_eq!(dup, String::from_str(&env, "You have already voted"));

        // Check results
        let poll = client.get_results();
        assert_eq!(poll.options.get(0).unwrap().vote_count, 1);
        assert_eq!(poll.options.get(1).unwrap().vote_count, 0);
    }

    #[test]
    fn test_close_poll() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PollContract);
        let client = PollContractClient::new(&env, &contract_id);

        client.create_poll(
            &String::from_str(&env, "Close poll test"),
            &String::from_str(&env, "Yes"),
            &String::from_str(&env, "No"),
            &String::from_str(&env, "Maybe"),
        );

        client.close_poll();

        let vote_after_close = client.vote(
            &String::from_str(&env, "GDEMO_WALLET_2"),
            &0,
        );
        assert_eq!(vote_after_close, String::from_str(&env, "Poll is no longer active"));
    }
}