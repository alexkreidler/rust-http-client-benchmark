use httpbench::*;
use std::time::{Duration, Instant};

fn main() {
    let urls = setup();

    let start = Instant::now();
    let out = <httpbench::Reqwest as Requestor>::make_reqs(urls.clone()).unwrap();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("{:#?}", out);
}