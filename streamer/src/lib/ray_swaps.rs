use serde::Serialize;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use super::ray_pool::RaydiumPool;

#[derive(Serialize, Clone, Debug)]
pub struct SwapInfo {
    timestamp: i64,
    input_token: Option<String>,
    input_amount: Option<u64>,
    output_token: Option<String>,
    output_amount: Option<u64>,
    implied_price: f64
}

pub fn extract_swap_details(tx: &EncodedConfirmedTransactionWithStatusMeta, ray_pool: &RaydiumPool) -> SwapInfo {
    let meta = tx.transaction.meta.as_ref().unwrap();
    let timestamp = tx.block_time.unwrap();
    let pre_token_balances = &meta.pre_token_balances.as_ref().unwrap();
    let post_token_balances = &meta.post_token_balances.as_ref().unwrap();
    let mut input_info = None;
    let mut output_info = None;

    for pre in pre_token_balances.iter() {
        if let Some(post) = post_token_balances.iter().find(|p| p.mint == pre.mint) {
            let pre_amount = pre.ui_token_amount.amount.parse::<f64>().unwrap_or(0.0);
            let post_amount = post.ui_token_amount.amount.parse::<f64>().unwrap_or(0.0);

            if post_amount < pre_amount {
                input_info = Some((
                    pre.mint.clone(),
                    (pre_amount - post_amount).abs() as u64
                ));
            } else if post_amount > pre_amount {
                output_info = Some((
                    pre.mint.clone(),
                    (post_amount - pre_amount).abs() as u64
                ));
            }


        }
    }

    let (input_token, input_amount) = if let Some(input_info) = input_info {
        (Some(input_info.0), Some(input_info.1))
    } else {
        (None, None)
    };
    let (output_token, output_amount) = if let Some(output_info) = output_info {
        (Some(output_info.0), Some(output_info.1))
    } else {
        (None, None)
    };
    
    SwapInfo {
        timestamp,
        input_token,
        input_amount,
        output_token,
        output_amount,
        implied_price: calculate_implied_price(ray_pool)
    }
}

pub fn calculate_implied_price(ray_pool: &RaydiumPool) -> f64 {
    let sol_balance = ray_pool.token_a.balance as f64;
    let token_balance = ray_pool.token_b.balance as f64;
    
    let sol_decimals = ray_pool.token_a.decimals;
    let token_decimals = ray_pool.token_b.decimals;
    
    let adjusted_sol = sol_balance / (10_f64.powi(sol_decimals as i32));
    let adjusted_token = token_balance / (10_f64.powi(token_decimals as i32));
    
    adjusted_token / adjusted_sol
}