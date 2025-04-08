use crate::car::Car;
use crate::driver::Driver;

#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub team_color: String,
    pub nationality: String,
    pub drivers: Vec<Driver>,
    pub car: Car,
}

impl Team {
    pub fn new(
        name: String,
        team_color: String,
        nationality: String,
        drivers: Vec<Driver>,
        car: Car,
    ) -> Team {
        Team {
            name,
            team_color,
            nationality,
            drivers,
            car,
        }
    }

    pub fn info(&self) -> String {
        let mut drivers_info = String::new();
        for driver in &self.drivers {
            drivers_info.push_str("    - ");
            drivers_info.push_str(&driver.full_name);
            drivers_info.push('\n');
        }

        format!(
            "🏎️ Team: {}\n🎨 Color: {}\n🌐 Nationality: {}\n👥 Drivers:\n{}{}",
            self.name,
            self.team_color,
            self.nationality,
            drivers_info,
            self.car.info()
        )
    }

    pub fn average_driver_rating(&self) -> f32 {
        let mut total = 0.0;

        for driver in &self.drivers {
            total += driver.overall_rating();
        }

        total / self.drivers.len() as f32
    }

    pub fn team_performance_score(&self) -> f32 {
        (self.car.overall_performance() + self.average_driver_rating()) / 2.0
    }
}