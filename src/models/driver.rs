use crate::models::Car;

#[derive(Debug, Clone)]
pub struct Driver {
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub name_acronym: String,
    pub age: u8,
    pub country: String,
    pub driver_number: u8,
    pub skill_level: u8,
    pub experience_level: u8,
    pub aggression_level: u8,
    pub consistency_level: u8,
    pub team_name: String,
}

impl Driver {
    pub fn new(
        first_name: String,
        last_name: String,
        full_name: String,
        name_acronym: String,
        age: u8,
        country: String,
        driver_number: u8,
        skill_level: u8,
        experience_level: u8,
        aggression_level: u8,
        consistency_level: u8,
        team_name: String,
    ) -> Driver {
        Driver {
            first_name,
            last_name,
            full_name,
            name_acronym,
            age,
            country,
            driver_number,
            skill_level,
            experience_level,
            aggression_level,
            consistency_level,
            team_name,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "🏁 Driver: {} ({})
            📛 Acronym: {}
            🌍 Country: {}
            🎂 Age: {}
            🔢 Number: {}
            🧠 Skill: {}
            📈 Experience: {}
            🔥 Aggression: {}
            🎯 Consistency: {}
            🏎Team: {}
            ",
            self.full_name,
            self.last_name,
            self.name_acronym,
            self.country,
            self.age,
            self.driver_number,
            self.skill_level,
            self.experience_level,
            self.aggression_level,
            self.consistency_level,
            self.team_name,
        )
    }

    /// Calculates overall driver rating percentage based on skill, experience, aggression, and consistency.
    pub fn overall_rating(&self) -> f32 {
        (self.skill_level as f32
            + self.experience_level as f32
            + self.consistency_level as f32
            + self.aggression_level as f32)
            / 100.0
    }

    /// Calculates a simulated lap time based on driver rating and car performance.
    pub fn simulated_lap_time(&self, car: &Car) -> f32 {
        let base_time = 90.0; // base time in seconds
        let driver_factor = 1.0 - self.overall_rating();
        let car_factor = 1.0 - (car.overall_performance() / 100.0);

        base_time * (driver_factor + car_factor) / 2.0
    }

    /// Determines if a driver is likely to cause a crash (DNF) based on aggression and car reliability.
    pub fn chance_of_dnf(&self, car: &Car) -> f32 {
        let base_chance = 0.02; // 2% base
        let aggression_factor = self.aggression_level as f32 / 100.0 * 0.15;
        let reliability_factor = (100 - car.reliability) as f32 / 100.0 * 0.1;

        base_chance + aggression_factor + reliability_factor
    }
}