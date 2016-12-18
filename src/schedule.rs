use curl::easy::Easy;
use serde_json::{self, Map, Value};
use chrono::*;
use game::Game;

pub struct Schedule {
    pub date: String,
    pub num_games: i32,
    pub games: Vec<Game>,
}

impl Schedule {
    pub fn today() -> Result<Schedule, &'static str> {
        let today_date = Local::today();
        Ok(Schedule::parse_from_api(today_date))
    }

    pub fn tomorrow() -> Result<Schedule, &'static str> {
        let today_date = Local::today();
        let tomorrow_date = Local.ymd(today_date.year(), today_date.month(), today_date.day() + 1);
        Ok(Schedule::parse_from_api(tomorrow_date))
    }

    pub fn yesterday() -> Result<Schedule, &'static str> {
        let today_date = Local::today();
        let yesterday_date = Local.ymd(today_date.year(), today_date.month(), today_date.day() - 1);
        Ok(Schedule::parse_from_api(yesterday_date))
    }

    fn parse_from_api(date: Date<Local>) -> Schedule {
        let mut buf = Vec::new();
        let today_date = date.format("%Y-%m-%d").to_string();
        {
            let url = format!("https://statsapi.web.nhl.com/api/v1/schedule?startDate={0}&endDate={0}", today_date);
            let mut curl_req = Easy::new();
            curl_req.url(&url).unwrap();

            let mut transfer = curl_req.transfer();
            transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        let json: Map<String, Value> = serde_json::from_slice(buf.as_mut_slice()).unwrap();

        // Find number of games.
        let num_games_v = json.get("totalGames").unwrap();
        // TODO: Removing will perform a move but maybe we should be caching?
        let num_games: i32 = serde_json::from_value(num_games_v.clone()).unwrap();

        // Find all games in the schedule.
        // Format: { dates: [ { games: [ ... ] } ] }

        // Get the array of dates from the JSON.
        let dates_v: &Value = json.get("dates").unwrap();
        // We only care about the first date since start_date == end_date.
        let date: &Map<String, Value> = dates_v.as_array().unwrap()[0].as_object().unwrap();
        // Now, from the first date object in the array, get the games array.
        let games_v: &Value = date.get("games").unwrap();
        // Map through each game scheduled and create a Game from the data.
        let mut games: Vec<Game> = games_v.as_array().unwrap().iter().map(|game_v| {
            let game: &Map<String, Value> = game_v.as_object().unwrap();

            let teams: &Map<String, Value> = game.get("teams").unwrap().as_object().unwrap();

            let home: &Map<String, Value> = teams.get("home").unwrap().as_object().unwrap();
            let home_goals: &Value = home.get("score").unwrap();
            let home_goals: i32 = serde_json::from_value(home_goals.clone()).unwrap();
            let home_team: &Map<String, Value> = home.get("team").unwrap().as_object().unwrap();
            let home_team: &Value = home_team.get("name").unwrap();
            let home_team: String = serde_json::from_value(home_team.clone()).unwrap();

            let away: &Map<String, Value> = teams.get("away").unwrap().as_object().unwrap();
            let away_goals: &Value = away.get("score").unwrap();
            let away_goals: i32 = serde_json::from_value(away_goals.clone()).unwrap();
            let away_team: &Map<String, Value> = away.get("team").unwrap().as_object().unwrap();
            let away_team: &Value = away_team.get("name").unwrap();
            let away_team: String = serde_json::from_value(away_team.clone()).unwrap();

            let status: &Map<String, Value> = game.get("status").unwrap().as_object().unwrap();
            let status_code: &Value = status.get("statusCode").unwrap();
            let status_code: String = serde_json::from_value(status_code.clone()).unwrap();
            let status_code = status_code.parse::<i32>().unwrap();

            Game::new(home_team, home_goals, away_team, away_goals, status_code)
        }).collect();
        games.sort();
        Schedule { date: today_date, num_games: num_games, games: games }
    }
}
