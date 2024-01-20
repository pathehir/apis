use serde::Deserialize;

#[derive(Deserialize)]
struct Quote {
    quote: String,
    author: String,
}

impl Quote {
    fn quote(&self) -> &str { &self.quote }
    fn author(&self) -> &str { &self.author }
}

fn get(num: Option<u32>) -> anyhow::Result<Quote> {
    Ok(ureq::get(format!("https://api.breakingbadquotes.xyz/v1/quotes/{}", num.unwrap_or(1)).as_str()).call()?.into_json()?)
}
