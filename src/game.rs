pub struct Game {
    home_team: String,
    home_goals: i32,
    away_team: String,
    away_goals: i32,
    status_code: i32,
}

impl Game {
    pub fn new(home_team: String, home_goals: i32, away_team: String, away_goals: i32, status_code: i32) -> Game {
        Game {
            home_team: home_team,
            home_goals: home_goals,
            away_team: away_team,
            away_goals: away_goals,
            status_code: status_code
        }
    }

    pub fn to_string(&self) -> String {
        let status_code = self.status_string();
        let home = format!("{} ({})", self.home_team, self.home_goals);
        let away = format!("{} ({})", self.away_team, self.away_goals);
        let home_str;
        let away_str;
        if self.status_code == 7 || self.status_code == 6 {
            home_str = match self.home_goals > self.away_goals {
                true => format!("\x0309{}\x03", home), // light green
                _ => format!("\x0304{}\x03", home) // red
            };
            away_str = match self.home_goals < self.away_goals {
                true => format!("\x0309{}\x03", away), // light green
                _ => format!("\x0304{}\x03", away) // red
            };
        } else if self.status_code == 3 {
            home_str = format!("\x0308{}\x03", home); // yellow
            away_str = format!("\x0308{}\x03", away); // yellow
        } else {
            home_str = self.home_team.clone();
            away_str = self.away_team.clone();
        }

        format!("{}: {} vs. {}", status_code, home_str, away_str)
    }

    fn status_string(&self) -> &str {
        match self.status_code {
            7 | 6 => "FINAL",
            3 => "LIVE",
            _ => "SCHEDULED"
        }
    }
}
