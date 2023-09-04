
use core::fmt;
use std::fmt::Display;

pub enum LeagueID {
    NBA,
}

pub struct Season(pub String);
pub struct LastNGames(pub i32);
pub struct GameID(pub String);
pub struct Period(pub i8);
pub struct StartPeriod(pub Period);
pub struct EndPeriod(pub Period);


impl Display for GameID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameID={}", self.0)
    }
}

impl Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Period={}", self.0)
        
    }
}

impl Display for StartPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StartPeriod={}", self.0.0)
        
    }
}

impl Display for EndPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EndPeriod={}", self.0.0)
    }
}

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Season={}", self.0)
    }
}

impl Display for LeagueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LeagueID::NBA => write!(f, "LeagueID=00")
        }
    }
}

impl Default for LeagueID {
    fn default() -> Self { LeagueID::NBA }
}

impl Default for Season {
    fn default() -> Self { 
        let current_date = chrono::Utc::now();
        let next_date = current_date - chrono::Duration::days(365);
        let first_year = next_date.format("%Y").to_string();
        let second_year: String = current_date.format("%y").to_string();
        Season(format!("{}-{}", first_year, second_year))
    }
}

impl Default for StartPeriod {
    fn default() -> Self { StartPeriod(Period(0)) }
}

impl Default for EndPeriod {
    fn default() -> Self { EndPeriod(Period(3)) }
}

impl Default for GameID {
    fn default() -> Self { GameID("".to_string()) }
}

impl Default for Period {
    fn default() -> Self { Period(0) }
}

impl Default for LastNGames {
    fn default() -> Self { LastNGames(0) }
}