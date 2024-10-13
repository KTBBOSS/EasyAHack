#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol, Address};

pub struct TradeFinanceContract;


#[contractimpl]
impl TradeFinanceContract {
    // ...
}
#[contractimpl]
impl TradeFinanceContract {
    pub fn init(env: Env, importer: Address, exporter: Address, total_amount: i128) {
        env.storage().persistent().set(&Symbol::new(&env, "importer"), &importer);
        env.storage().persistent().set(&Symbol::new(&env, "exporter"), &exporter);
        env.storage().persistent().set(&Symbol::new(&env, "total_amount"), &total_amount);
        env.storage().persistent().set(&Symbol::new(&env, "released_amount"), &0i128);
    }

    pub fn release_funds(env: Env, checkpoint: u32, amount: i128) {
        let _importer: Address = env.storage().persistent().get(&Symbol::new(&env, "importer")).unwrap();
        let _exporter: Address = env.storage().persistent().get(&Symbol::new(&env, "exporter")).unwrap();
        let total_amount: i128 = env.storage().persistent().get(&Symbol::new(&env, "total_amount")).unwrap();
        let mut released_amount: i128 = env.storage().persistent().get(&Symbol::new(&env, "released_amount")).unwrap();

        if released_amount + amount <= total_amount {
            released_amount += amount;
            env.storage().persistent().set(&Symbol::new(&env, "released_amount"), &released_amount);

            env.events().publish(
                (Symbol::new(&env, "payment_made"), checkpoint),
                amount,
            );
        } else {
            panic!("Amount exceeds total or checkpoint already processed.");
        }
    }

    pub fn check_status(env: Env) -> i128 {
        env.storage().persistent().get(&Symbol::new(&env, "released_amount")).unwrap_or(0)
    }
}