use rand::Rng;

#[derive(Debug, Clone)]
pub struct Car {
    pub name: String,
    pub engine_power: u8,
    pub tyre_management: u8,
    pub reliability: u8,
    pub aerodynamics: u8,
}

impl Car {
    pub fn new(
        name: String,
        engine_power: u8,
        tyre_management: u8,
        reliability: u8,
        aerodynamics: u8,
    ) -> Car {
        Car {
            name,
            engine_power,
            tyre_management,
            reliability,
            aerodynamics,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "🚗 Car: {}
            ⚙️ Engine Power: {}
            🛞 Tyre Management: {}
            🔧 Reliability: {}
            🪂 Aerodynamics: {}
            ",
            self.name,
            self.engine_power,
            self.tyre_management,
            self.reliability,
            self.aerodynamics
        )
    }

    pub fn overall_performance(&self) -> f32 {
        let random_number = rand::thread_rng().gen_range(0.0..1.0);

        self.engine_power as f32 * random_number
            + self.aerodynamics as f32 * random_number
            + self.tyre_management as f32 * random_number
            + self.reliability as f32 * random_number
    }
}