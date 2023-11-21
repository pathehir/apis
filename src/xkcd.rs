use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Xkcd {
    pub month: String,
    pub num: u32,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

impl Xkcd {
    pub fn get_latest() -> anyhow::Result<Self> {
        Ok(ureq::get("https://xkcd.com/info.0.json").call()?.into_json()?)
    }

    pub fn get(num: u32) -> anyhow::Result<Self> {
        Ok(ureq::get(format!("https://xkcd.com/{num}/info.0.json").as_str()).call()?.into_json()?)
    }
}
