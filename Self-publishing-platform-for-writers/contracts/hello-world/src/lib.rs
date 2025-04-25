#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Structure to represent a published work
#[contracttype]
#[derive(Clone)]
pub struct PublishedWork {
    pub unique_id: u64,
    pub title: String,
    pub author: String,
    pub content: String,
    pub published_time: u64,
}

// Mapping for published works
#[contracttype]
pub enum Workbook {
    Work(u64)
}

// Counter for unique work IDs
const COUNT_WORK: Symbol = symbol_short!("C_WORK");

#[contract]
pub struct SelfPublishingContract;

#[contractimpl]
impl SelfPublishingContract {
    // Function to publish a new work
    pub fn publish_work(env: Env, title: String, author: String, content: String) -> u64 {
        let mut count_work: u64 = env.storage().instance().get(&COUNT_WORK).unwrap_or(0);
        count_work += 1;

        let time = env.ledger().timestamp();

        // Create a new published work
        let new_work = PublishedWork {
            unique_id: count_work,
            title,
            author,
            content,
            published_time: time,
        };

        // Store the published work
        env.storage().instance().set(&Workbook::Work(count_work), &new_work);
        env.storage().instance().set(&COUNT_WORK, &count_work);

        log!(&env, "Work Published with ID: {}", count_work);
        count_work // Return the unique ID of the published work
    }

    // Function to view a published work by its unique ID
    pub fn view_work(env: Env, unique_id: u64) -> PublishedWork {
        let key = Workbook::Work(unique_id);
        env.storage().instance().get(&key).unwrap_or(PublishedWork {
            unique_id: 0,
            title: String::from_str(&env, "Not Found"),
            author: String::from_str(&env, "Not Found"),
            content: String::from_str(&env, "Not Found"),
            published_time: 0,
        })
    }

    // Function to get the total number of published works
    pub fn total_published_works(env: Env) -> u64 {
        env.storage().instance().get(&COUNT_WORK).unwrap_or(0)
    }
}