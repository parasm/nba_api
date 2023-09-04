use serde_json::Value;

use crate::endpoint::NBAStatsEndpoint;
use crate::params::{GameID,StartPeriod, EndPeriod};

pub struct PlayByPlayV2 {
    game_id: GameID,
    start_period: StartPeriod,
    end_period: EndPeriod,
}

impl NBAStatsEndpoint for PlayByPlayV2 {
    fn endpoint_params(&self) -> String {
        format!("{}&{}&{}", self.game_id, self.start_period, self.end_period)
    }
    fn endpoint_name(&self) -> String {
        "playbyplayv2".to_string()
    }
}

impl PlayByPlayV2 {
    pub fn fetch_plays_for_game(game_id: GameID) -> Value {
        let pbpv2 = PlayByPlayV2 {
            game_id : game_id,
            start_period : StartPeriod::default(),
            end_period : EndPeriod::default(),
        };
        
        pbpv2.fetch_nba_json()
    }
}