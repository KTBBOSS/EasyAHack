#![cfg(test)]

use soroban_sdk::{Env, Symbol, Address};
use trade_finance_contract::TradeFinance;

#[test]
fn test_trade_finance() {
    let env = Env::default();
    let importer = Address::generate(&env);
    let exporter = Address::generate(&env);
    let total_amount = 90i128;

    TradeFinance::init(&env, &importer, &exporter, &total_amount);

    let checkpoints = vec!["Shipment Left Port", "Shipment Crossed Border", "Shipment Arrived"];
    for (i, checkpoint) in checkpoints.iter().enumerate() {
        println!("Checkpoint {} reached: {}", i + 1, checkpoint);

        let release_amount = 30i128;
        TradeFinance::release_funds(&env, i as u32, release_amount);

        let released_amount = TradeFinance::check_status(&env);
        println!("Total released after checkpoint {}: {}", i + 1, released_amount);
    }

    println!("Final payment after goods are received.");
    let final_funds = 30i128;
    TradeFinance::release_funds(&env, 3, final_funds);

    let final_status = TradeFinance::check_status(&env);
    println!("Transaction complete, total funds released: {}", final_status);

    assert_eq!(final_status, total_amount);
}