use serde::Deserialize;
use crate::Result;

#[derive(Deserialize)]
pub struct Score {
    pub score: i32,
    pub name: String,
    pub timestamp: i64
}

#[derive(Deserialize)]
pub struct Scores {
    all_time: Vec<Score>,
    past_24hours: Vec<Score>,
    past_week: Vec<Score>,
    past_30days: Vec<Score>
}

pub enum ScoreList {
    AllTime,
    Past24Hours,
    PastWeek,
    Past30Days,
}

impl Scores {
    pub fn get(list: ScoreList) -> Result<Vec<Score>> {
        let lists: Scores = ureq::get("https://highscores.neonrogue.net/scores").call()?.into_json()?;

        let l = match list {
            ScoreList::AllTime => lists.all_time,
            ScoreList::Past24Hours => lists.past_24hours,
            ScoreList::PastWeek => lists.past_week,
            ScoreList::Past30Days => lists.past_30days,
        };

        log::info!("successfully recieved neonrogue scores.");

        Ok(l)
    }
}
