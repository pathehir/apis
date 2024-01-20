use serde::Deserialize;

#[derive(Deserialize)]
pub struct Quote {
    quote: String,
    author: String,
}

impl Quote {
    pub fn quote(&self) -> &str { &self.quote }
    pub fn author(&self) -> &str { &self.author }

    pub fn get(num: Option<u32>) -> anyhow::Result<Vec<Quote>> {
        Ok(ureq::get(format!("https://api.breakingbadquotes.xyz/v1/quotes/{}", num.unwrap_or(1)).as_str()).call()?.into_json()?)
    }
}
