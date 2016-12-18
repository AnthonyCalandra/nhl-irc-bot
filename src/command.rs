use schedule::Schedule;

fn get_scores(schedule: Schedule) -> Vec<String> {
    let mut messages: Vec<String> = vec![];
    let header = format!("{} scheduled games during {}:", schedule.num_games, schedule.date);
    messages.push(header);
    for game in &schedule.games {
        let msg = format!("{}", game.to_string());
        messages.push(msg);
    }

    messages
}

pub fn execute(command: &str, args: Vec<&str>) -> Result<Vec<String>, &'static str> {
    if command == "help" {
        let msg = vec![
            "The following commands are available:".to_string(),
            "Get full schedule of games: scores|games [yesterday|tomorrow|$y-$m-$d]".to_string(),
            "Get schedule of games that match keywords: score|game [keywords...]".to_string()
        ];
        return Ok(msg);
    }


    if command == "scores" || command == "games" {
        let schedule = match args[0] {
            "" => Schedule::today(),
            "yesterday" => Schedule::yesterday(),
            "tomorrow" => Schedule::tomorrow(),
            _ => return Err("Invalid argument.")
        };
        return Ok(get_scores(schedule.unwrap()));
    }

    if command == "score" || command == "game" {
        if args[0] == "" {
            return Err("No search keywords provided.");
        }

        let args: Vec<String> = args.iter().map(|keyword| keyword.to_lowercase()).collect();
        let mut games = get_scores(Schedule::today().unwrap());
        games.retain(|ref game| {
            let game = game.to_lowercase();
            args.iter().any(|keyword| game.find(keyword).is_some())
        });
        return Ok(games);
    }

    Err("Unrecognized command.")
}
