#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token,
    Address, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SplitpayError {
    SplitAlreadyExists = 1,
    SplitNotFound = 2,
    InvalidAmount = 3,
    InvalidParticipants = 4,
    SplitAlreadyCompleted = 5,
    ParticipantAlreadyPaid = 6,
    MathOverflow = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Split {
    pub id: String,
    pub receiver: Address,
    pub token: Option<Address>,
    pub total_amount: i128,
    pub num_participants: u32,
    pub amount_per_person: i128,
    pub payments_received: u32,
    pub is_completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRecord {
    pub split_id: String,
    pub payer: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Split(String),
    Payment(String, Address),
}

#[contract]
pub struct SplitpayContract;

#[contractimpl]
impl SplitpayContract {
    /// Initialize a new split session
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `creator` - Address creating the split (must authenticate)
    /// * `split_id` - Unique identifier for the split session
    /// * `receiver` - Address that will receive funds
    /// * `token` - Optional token contract address (if None, ledger accounting only)
    /// * `total_amount` - Total amount to be split
    /// * `num_participants` - Total number of participants
    ///
    /// # Returns
    /// The amount each participant should pay
    pub fn init_split(
        env: Env,
        creator: Address,
        split_id: String,
        receiver: Address,
        token: Option<Address>,
        total_amount: i128,
        num_participants: u32,
    ) -> Result<i128, SplitpayError> {
        creator.require_auth();

        if total_amount <= 0 {
            return Err(SplitpayError::InvalidAmount);
        }

        if num_participants < 2 {
            return Err(SplitpayError::InvalidParticipants);
        }

        let key = DataKey::Split(split_id.clone());
        if env.storage().persistent().has(&key) {
            return Err(SplitpayError::SplitAlreadyExists);
        }

        let amount_per_person = total_amount
            .checked_div(num_participants as i128)
            .ok_or(SplitpayError::MathOverflow)?;

        if amount_per_person <= 0 {
            return Err(SplitpayError::InvalidAmount);
        }

        let split = Split {
            id: split_id.clone(),
            receiver: receiver.clone(),
            token,
            total_amount,
            num_participants,
            amount_per_person,
            payments_received: 0,
            is_completed: false,
        };

        env.storage().persistent().set(&key, &split);

        env.events().publish(
            (symbol_short!("init_splt"), split_id),
            (receiver, total_amount, amount_per_person),
        );

        Ok(amount_per_person)
    }

    /// Record a payment from a participant
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `split_id` - The split session ID
    /// * `payer` - Address of the person making the payment (must authenticate)
    /// * `amount` - Amount being paid (must match amount_per_person)
    ///
    /// # Returns
    /// True if all payments have now been received, false otherwise
    pub fn record_payment(
        env: Env,
        split_id: String,
        payer: Address,
        amount: i128,
    ) -> Result<bool, SplitpayError> {
        payer.require_auth();

        let split_key = DataKey::Split(split_id.clone());
        let mut split: Split = env
            .storage()
            .persistent()
            .get(&split_key)
            .ok_or(SplitpayError::SplitNotFound)?;

        if split.is_completed {
            return Err(SplitpayError::SplitAlreadyCompleted);
        }

        if amount != split.amount_per_person {
            return Err(SplitpayError::InvalidAmount);
        }

        let pay_key = DataKey::Payment(split_id.clone(), payer.clone());
        if env.storage().persistent().has(&pay_key) {
            return Err(SplitpayError::ParticipantAlreadyPaid);
        }

        // If a token contract is configured, transfer funds from payer to receiver
        if let Some(ref token_addr) = split.token {
            let token_client = token::Client::new(&env, token_addr);
            token_client.transfer(&payer, &split.receiver, &amount);
        }

        // Record payment
        env.storage().persistent().set(&pay_key, &amount);

        split.payments_received += 1;
        if split.payments_received >= split.num_participants {
            split.is_completed = true;
        }

        env.storage().persistent().set(&split_key, &split);

        let payment_record = PaymentRecord {
            split_id: split_id.clone(),
            payer: payer.clone(),
            amount,
        };

        env.events().publish(
            (symbol_short!("payment"), split_id),
            (payment_record.payer, payment_record.amount, split.payments_received),
        );

        Ok(split.is_completed)
    }

    /// Get current status of a split session
    pub fn get_split_status(env: Env, split_id: String) -> Result<Split, SplitpayError> {
        let key = DataKey::Split(split_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(SplitpayError::SplitNotFound)
    }

    /// Check if a split is complete
    pub fn is_split_complete(env: Env, split_id: String) -> Result<bool, SplitpayError> {
        let split = Self::get_split_status(env, split_id)?;
        Ok(split.is_completed)
    }

    /// Check if a specific participant has paid
    pub fn has_participant_paid(env: Env, split_id: String, payer: Address) -> bool {
        let pay_key = DataKey::Payment(split_id, payer);
        env.storage().persistent().has(&pay_key)
    }
}

#[cfg(test)]
mod test;
