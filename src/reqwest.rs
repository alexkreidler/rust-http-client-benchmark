use tokio;
use tokio::runtime::Runtime;
use futures::future::join_all;

use reqwest;
use anyhow::{Error};

pub struct Reqwest {}

use super::Meta;

impl super::Requestor for Reqwest {
    fn make_reqs(urls_batch: Vec<String>) -> Result<Meta, Error> {
        // println!("URL 1: {}", urls_batch[0]);
        let mut rt = Runtime::new()?;

        let work = rt.block_on(async {
            let mut t = Vec::new();
            for url in urls_batch {
                t.push(tokio::task::spawn(async move {
                    reqwest::get(url.as_str()).await?.text().await.or_else(|e| Err(Error::new(e).context(format!("failed to get url: {}", url))))
                }))
            }
            
            join_all(t).await
        });

        let mut total: usize = 0;
        let mut completed = 0;
        for res in work {
            let text: String = res??;
            completed += 1;
            total += text.len();
        }
        Ok(Meta{ total, completed })
    }
}