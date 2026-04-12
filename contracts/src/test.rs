#[cfg(test)]
mod tests {
    use crate::{SplitpayContract, SplitpayContractClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_init_split() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SplitpayContract);
        let client = SplitpayContractClient::new(&env, &contract_id);

        let receiver = Address::random(&env);
        let split_id = String::from_slice(&env, "split-001");
        let total_amount: i128 = 40_000_000; // 40 XLM (in stroops)
        let num_participants: u32 = 4;

        let amount_per_person = client.init_split(
            &split_id,
            &receiver,
            &total_amount,
            &num_participants,
        );

        assert_eq!(amount_per_person, 10_000_000);
    }

    #[test]
    fn test_record_payment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SplitpayContract);
        let client = SplitpayContractClient::new(&env, &contract_id);

        let receiver = Address::random(&env);
        let split_id = String::from_slice(&env, "split-001");
        
        // Initialize split
        client.init_split(
            &split_id,
            &receiver,
            &40_000_000,
            &4,
        );

        // Record payment from participant
        let payer = Address::random(&env);
        let is_complete = client.record_payment(
            &split_id,
            &payer,
            &10_000_000,
        );

        assert_eq!(is_complete, false); // Not complete yet (1 of 4)

        // Get status
        let status = client.get_split_status();
        assert_eq!(status.payments_received, 1);
    }
}
