#[derive(Debug, Clone)]
pub struct RaceResult {
    pub driver_name: String,
    pub team_name: String,
    pub finish_position: u8,
    pub finish_time_seconds: u8,
    pub dnf: bool,
}

impl RaceResult {
    pub fn new(
        driver_name: String,
        team_name: String,
        finish_position: u8,
        finish_time_seconds: u8,
        dnf: bool,
    ) -> RaceResult {
        RaceResult {
            driver_name,
            team_name,
            finish_position,
            finish_time_seconds,
            dnf,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "🏁 Race Result
            👤 Driver: {}
            🏢 Team: {}
            🏆 Finish Position: {}
            ⏱️ Finish Time: {}s
            ❌ DNF: {}
            ",
            self.driver_name,
            self.team_name,
            self.finish_position,
            self.finish_time_seconds,
            if self.dnf { "Yes" } else { "No" }
        )
    }
}