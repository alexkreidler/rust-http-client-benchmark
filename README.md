# Rust HTTP Client Benchmark

The goal of this project is to analyze the performance of various Rust HTTP client libraries.

Many of them utilize the new `async` ecosystem, either through `tokio`, `async-std` or `actix`.

I was curious whether `async`/`Future` features really have any effect on performance on the client side.

These benchmarks are written using `Criterion`, which allows for paramaterization and nice reporting.

The following libraries are tested: `reqwest ureq isahc http_req attohttpc minreq cabot surf awc`

As of now, only Reqwest has been implemented.

It turns out criterion might not be the best for testing long/resource intensive (e.g. network IO) tasks.

May need to figure out other benchmarking library or own tool.

This project was mainly inspired by:
https://medium.com/@shnatsel/smoke-testing-rust-http-clients-b8f2ee5db4e6
http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust

It was also partially inspired by the larger ecosystem of download tools, such as:
https://github.com/hashicorp/go-getter
https://github.com/rclone/rclone