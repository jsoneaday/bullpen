use borsh::{BorshDeserialize, BorshSerialize};
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
pub struct RaydiumPool {
    pub nonce: u8,
    pub amp_factor: u64,
    pub total_amount: u64,
    pub token_a: TokenInfo,
    pub token_b: TokenInfo,
    pub fees: PoolFees
}

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize, Serialize)]
pub struct PoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub protocol_fee_numerator: u64,
    pub protocol_fee_denominator: u64,
}

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize, Serialize)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub balance: u64,
    pub decimals: u8,
}

impl RaydiumPool {
    pub fn decode(data: &[u8]) -> Result<Self, borsh::io::Error> {
        RaydiumPool::deserialize(&mut data.as_ref())
    }

    pub fn calculate_swap_amount(&self, input_amount: u64, is_a_to_b: bool) -> Option<u64> {
        let (input_balance, output_balance) = if is_a_to_b {
            (self.token_a.balance, self.token_b.balance)
        } else {
            (self.token_b.balance, self.token_a.balance)
        };

        let fee_adjusted_input = input_amount * (self.fees.trade_fee_denominator - self.fees.trade_fee_numerator) 
            / self.fees.trade_fee_denominator;
        
        let output_amount = (output_balance * fee_adjusted_input) 
            / (input_balance + fee_adjusted_input);

        Some(output_amount)
    }
}