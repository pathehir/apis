use serde::Deserialize;
use crate::Result;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Comic {
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

impl Comic {
    pub fn get(num: Option<u32>) -> Result<Self> {
        let c = match num {
            Some(n) => Ok(ureq::get(format!("https://xkcd.com/{n}/info.0.json").as_str()).call()?.into_json()?),
            None => Ok(ureq::get("https://xkcd.com/info.0.json").call()?.into_json()?),
        };

        log::info!("successfully recieved comic");

        c
    }
}
