use serde::Deserialize;
use crate::Result;

#[derive(Deserialize)]
pub struct Quote {
    pub quote: String,
    pub author: String,
}

impl Quote {
    pub fn get(num: Option<u32>) -> Result<Vec<Quote>> {
        let q = match num {
            Some(n) => Ok(ureq::get(format!("https://api.breakingbadquotes.xyz/v1/quotes/{n}").as_str()).call()?.into_json()?),
            None => Ok(ureq::get("https://api.breakingbadquotes.xyz/v1/quotes").call()?.into_json()?)
        };

        log::info!("successfully recieved quote.");

        q
    }
}
