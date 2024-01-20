use serde::Deserialize;

#[derive(Deserialize)]
pub struct Score {
    score: i32,
    name: String,
    timestamp: i64
}

impl Score {
    pub fn score(&self) -> i32 { self.score }
    pub fn name(&self) -> &str { &self.name }
    pub fn timestamp(&self) -> i64 { self.timestamp }
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

enum ListType {
    All(Scores),
    One(Vec<Score>),
}

impl Scores {
    pub fn get(list: ScoreList) -> anyhow::Result<Vec<Score>> {
        let lists: Scores = ureq::get("https://highscores.neonrogue.net/scores").call()?.into_json()?;

        Ok(match list {
            ScoreList::AllTime => lists.all_time,
            ScoreList::Past24Hours => lists.past_24hours,
            ScoreList::PastWeek => lists.past_week,
            ScoreList::Past30Days => lists.past_30days,
        })
    }
}
