#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Map, symbol_short, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Split {
    pub id: String,
    pub receiver: Address,
    pub total_amount: i128,
    pub num_participants: u32,
    pub amount_per_person: i128,
    pub payments_received: u32,
    pub is_completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    pub split_id: String,
    pub payer: Address,
    pub amount: i128,
}

#[contract]
pub struct SplitpayContract;

#[contractimpl]
impl SplitpayContract {
    /// Initialize a new split session
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `split_id` - Unique identifier for the split
    /// * `receiver` - Address that will receive the funds
    /// * `total_amount` - Total amount to be split
    /// * `num_participants` - Number of people in the split
    ///
    /// # Returns
    /// The amount each participant should pay
    pub fn init_split(
        env: Env,
        split_id: String,
        receiver: Address,
        total_amount: i128,
        num_participants: u32,
    ) -> i128 {
        let amount_per_person = total_amount / num_participants as i128;

        let split = Split {
            id: split_id.clone(),
            receiver: receiver.clone(),
            total_amount,
            num_participants,
            amount_per_person,
            payments_received: 0,
            is_completed: false,
        };

        env.storage()
            .persistent()
            .set(&symbol_short!("split"), &split);

        env.events().publish(
            (symbol_short!("init"), split_id),
            split,
        );

        amount_per_person
    }

    /// Record a payment from a participant
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `split_id` - The split session ID
    /// * `payer` - Address of the person making the payment
    /// * `amount` - Amount being paid
    pub fn record_payment(
        env: Env,
        split_id: String,
        payer: Address,
        amount: i128,
    ) -> bool {
        payer.require_auth();

        let mut split: Split = env
            .storage()
            .persistent()
            .get(&symbol_short!("split"))
            .expect("Split not found")
            .unwrap();

        // Validate payment amount
        if amount != split.amount_per_person {
            env.panic_with_error(symbol_short!("invalid_amnt"));
        }

        // Check if already completed
        if split.is_completed {
            env.panic_with_error(symbol_short!("completed"));
        }

        // Update payment count
        split.payments_received += 1;

        // Check if all payments received
        if split.payments_received >= split.num_participants {
            split.is_completed = true;
        }

        env.storage()
            .persistent()
            .set(&symbol_short!("split"), &split);

        // Record payment event
        let payment = Payment {
            split_id: split_id.clone(),
            payer: payer.clone(),
            amount,
        };

        env.events().publish(
            (symbol_short!("payment"), split_id),
            payment,
        );

        split.is_completed
    }

    /// Get the current status of a split
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// The Split data with current status
    pub fn get_split_status(env: Env) -> Split {
        env.storage()
            .persistent()
            .get(&symbol_short!("split"))
            .expect("Split not found")
            .unwrap()
    }

    /// Check if a split is complete
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// True if all payments received, false otherwise
    pub fn is_split_complete(env: Env) -> bool {
        let split: Split = env
            .storage()
            .persistent()
            .get(&symbol_short!("split"))
            .expect("Split not found")
            .unwrap();

        split.is_completed
    }
}

mod test;
