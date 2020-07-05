#![warn(clippy::all)]

// Example to illustrate use of Criterion
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
use anyhow::Error;

// type MyErr = Box<dyn Error + 'static + Sync + Send>;

// pub trait Requestor {
//     fn make_req(url: &str) -> Result<usize, MyErr>;
// }

// // async traits require extra costs for each call, not worth for benchmark
// pub trait AsyncRequestor {
//     fn make_req(url: &str) -> Result<usize, MyErr>;
// }

// pub trait BatchRequestor
// loops over make_req

#[derive(Debug)]
pub struct Meta {
    completed: i32,
    total: usize,
}

pub trait Requestor {
    // This function does all the requests in a batch.
    // It should return the total number of bytes recieved
    // For async clients, it also includes the time to setup the executor, etc
    fn make_reqs(urls_batch: Vec<String>) -> Result<Meta, Error>;
}

pub mod reqwest;
pub mod utils;

pub use crate::reqwest::*;
pub use crate::utils::*;
