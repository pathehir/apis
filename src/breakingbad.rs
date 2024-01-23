use serde::Deserialize;
use crate::Result;

#[derive(Deserialize)]
pub struct Quote {
    quote: String,
    author: String,
}

impl Quote {
    pub fn quote(&self) -> &str { &self.quote }
    pub fn author(&self) -> &str { &self.author }

    pub fn get(num: Option<u32>) -> Result<Vec<Quote>> {
        match num {
            Some(n) => Ok(ureq::get(format!("https://api.breakingbadquotes.xyz/v1/quotes/{n}").as_str()).call()?.into_json()?),
            None => Ok(ureq::get("https://api.breakingbadquotes.xyz/v1/quotes").call()?.into_json()?)
        }
    }
}
