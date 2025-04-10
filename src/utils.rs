use crate::models::{Car, Driver, RaceResult, Team};
use rand::Rng;

pub fn create_drivers() -> Vec<Driver> {
    vec![
        Driver::new(
            "Max".to_string(),
            "Verstappen".to_string(),
            "Max Verstappen".to_string(),
            "VER".to_string(),
            26,
            "Netherlands".to_string(),
            1,
            95,
            90,
            85,
            90,
            "Oracle Red Bull Racing".to_string(),
        ),
        Driver::new(
            "Sergio".to_string(),
            "Perez".to_string(),
            "Sergio Perez".to_string(),
            "PER".to_string(),
            34,
            "Mexico".to_string(),
            11,
            88,
            85,
            80,
            85,
            "Oracle Red Bull Racing".to_string(),
        ),
        Driver::new(
            "Lewis".to_string(),
            "Hamilton".to_string(),
            "Lewis Hamilton".to_string(),
            "HAM".to_string(),
            39,
            "United Kingdom".to_string(),
            44,
            92,
            98,
            75,
            95,
            "Mercedes".to_string(),
        ),
        Driver::new(
            "George".to_string(),
            "Russell".to_string(),
            "George Russell".to_string(),
            "RUS".to_string(),
            26,
            "United Kingdom".to_string(),
            63,
            89,
            85,
            78,
            88,
            "Mercedes".to_string(),
        ),
        Driver::new(
            "Charles".to_string(),
            "Leclerc".to_string(),
            "Charles Leclerc".to_string(),
            "LEC".to_string(),
            26,
            "Monaco".to_string(),
            16,
            91,
            87,
            82,
            86,
            "Scuderia Ferrari".to_string(),
        ),
        Driver::new(
            "Carlos".to_string(),
            "Sainz".to_string(),
            "Carlos Sainz".to_string(),
            "SAI".to_string(),
            29,
            "Spain".to_string(),
            55,
            88,
            89,
            76,
            87,
            "Scuderia Ferrari".to_string(),
        ),
        Driver::new(
            "Lando".to_string(),
            "Norris".to_string(),
            "Lando Norris".to_string(),
            "NOR".to_string(),
            24,
            "United Kingdom".to_string(),
            4,
            90,
            85,
            79,
            89,
            "McLaren".to_string(),
        ),
        Driver::new(
            "Oscar".to_string(),
            "Piastri".to_string(),
            "Oscar Piastri".to_string(),
            "PIA".to_string(),
            23,
            "Australia".to_string(),
            81,
            84,
            78,
            74,
            83,
            "McLaren".to_string(),
        ),
        Driver::new(
            "Fernando".to_string(),
            "Alonso".to_string(),
            "Fernando Alonso".to_string(),
            "ALO".to_string(),
            42,
            "Spain".to_string(),
            14,
            90,
            99,
            70,
            92,
            "Aston Martin".to_string(),
        ),
        Driver::new(
            "Lance".to_string(),
            "Stroll".to_string(),
            "Lance Stroll".to_string(),
            "STR".to_string(),
            25,
            "Canada".to_string(),
            18,
            80,
            78,
            76,
            79,
            "Aston Martin".to_string(),
        ),
        Driver::new(
            "Esteban".to_string(),
            "Ocon".to_string(),
            "Esteban Ocon".to_string(),
            "OCO".to_string(),
            27,
            "France".to_string(),
            31,
            84,
            83,
            77,
            84,
            "Alpine".to_string(),
        ),
        Driver::new(
            "Pierre".to_string(),
            "Gasly".to_string(),
            "Pierre Gasly".to_string(),
            "GAS".to_string(),
            28,
            "France".to_string(),
            10,
            85,
            84,
            78,
            82,
            "Alpine".to_string(),
        ),
        Driver::new(
            "Alex".to_string(),
            "Albon".to_string(),
            "Alex Albon".to_string(),
            "ALB".to_string(),
            28,
            "Thailand".to_string(),
            23,
            86,
            82,
            79,
            84,
            "Williams".to_string(),
        ),
        Driver::new(
            "Logan".to_string(),
            "Sargeant".to_string(),
            "Logan Sargeant".to_string(),
            "SAR".to_string(),
            24,
            "USA".to_string(),
            2,
            75,
            70,
            80,
            75,
            "Williams".to_string(),
        ),
        Driver::new(
            "Kevin".to_string(),
            "Magnussen".to_string(),
            "Kevin Magnussen".to_string(),
            "MAG".to_string(),
            31,
            "Denmark".to_string(),
            20,
            82,
            81,
            82,
            78,
            "Haas".to_string(),
        ),
        Driver::new(
            "Nico".to_string(),
            "Hulkenberg".to_string(),
            "Nico Hulkenberg".to_string(),
            "HUL".to_string(),
            36,
            "Germany".to_string(),
            27,
            83,
            85,
            75,
            80,
            "Haas".to_string(),
        ),
        Driver::new(
            "Valtteri".to_string(),
            "Bottas".to_string(),
            "Valtteri Bottas".to_string(),
            "BOT".to_string(),
            35,
            "Finland".to_string(),
            77,
            85,
            90,
            70,
            85,
            "Stake F1 Team Kick Sauber".to_string(),
        ),
        Driver::new(
            "Zhou".to_string(),
            "Guanyu".to_string(),
            "Zhou Guanyu".to_string(),
            "ZHO".to_string(),
            25,
            "China".to_string(),
            24,
            80,
            78,
            75,
            77,
            "Stake F1 Team Kick Sauber".to_string(),
        ),
        Driver::new(
            "Yuki".to_string(),
            "Tsunoda".to_string(),
            "Yuki Tsunoda".to_string(),
            "TSU".to_string(),
            24,
            "Japan".to_string(),
            22,
            82,
            80,
            83,
            78,
            "Visa Cash App RB".to_string(),
        ),
        Driver::new(
            "Daniel".to_string(),
            "Ricciardo".to_string(),
            "Daniel Ricciardo".to_string(),
            "RIC".to_string(),
            35,
            "Australia".to_string(),
            3,
            87,
            88,
            74,
            83,
            "Visa Cash App RB".to_string(),
        ),
    ]
}

pub fn create_cars() -> Vec<(String, Car)> {
    vec![
        (
            "Oracle Red Bull Racing".to_string(),
            Car {
                name: "RB20".to_string(),
                engine_power: 95,
                tyre_management: 88,
                reliability: 92,
                aerodynamics: 93,
            },
        ),
        (
            "Mercedes".to_string(),
            Car {
                name: "W15".to_string(),
                engine_power: 90,
                tyre_management: 85,
                reliability: 88,
                aerodynamics: 87,
            },
        ),
        (
            "Scuderia Ferrari".to_string(),
            Car {
                name: "SF-24".to_string(),
                engine_power: 89,
                tyre_management: 83,
                reliability: 87,
                aerodynamics: 85,
            },
        ),
        (
            "McLaren".to_string(),
            Car {
                name: "MCL38".to_string(),
                engine_power: 88,
                tyre_management: 84,
                reliability: 86,
                aerodynamics: 89,
            },
        ),
        (
            "Aston Martin".to_string(),
            Car {
                name: "AMR24".to_string(),
                engine_power: 86,
                tyre_management: 80,
                reliability: 85,
                aerodynamics: 82,
            },
        ),
        (
            "Alpine".to_string(),
            Car {
                name: "A524".to_string(),
                engine_power: 83,
                tyre_management: 78,
                reliability: 82,
                aerodynamics: 80,
            },
        ),
        (
            "Williams".to_string(),
            Car {
                name: "FW46".to_string(),
                engine_power: 82,
                tyre_management: 77,
                reliability: 81,
                aerodynamics: 79,
            },
        ),
        (
            "Haas".to_string(),
            Car {
                name: "VF-24".to_string(),
                engine_power: 81,
                tyre_management: 75,
                reliability: 80,
                aerodynamics: 78,
            },
        ),
        (
            "Stake F1 Team Kick Sauber".to_string(),
            Car {
                name: "C44".to_string(),
                engine_power: 80,
                tyre_management: 74,
                reliability: 79,
                aerodynamics: 76,
            },
        ),
        (
            "Visa Cash App RB".to_string(),
            Car {
                name: "VCARB 01".to_string(),
                engine_power: 82,
                tyre_management: 76,
                reliability: 80,
                aerodynamics: 77,
            },
        ),
    ]
}

pub fn create_teams(drivers: &Vec<Driver>, cars: &Vec<(String, Car)>) -> Vec<Team> {
    let mut teams = Vec::new();

    // Basic team metadata
    // (team_name, color, nationality)
    let team_info = vec![
        ("Oracle Red Bull Racing", "001344", "Austrian"),
        ("Mercedes", "C0C0C0", "German"),
        ("Scuderia Ferrari", "FF2800", "Italian"),
        ("McLaren", "FF8700", "British"),
        ("Aston Martin", "006F62", "British"),
        ("Alpine", "005BAA", "French"),
        ("Williams", "005AFF", "British"),
        ("Haas", "FFFFFF", "American"),
        ("Stake F1 Team Kick Sauber", "4CBB17", "Swiss"),
        ("Visa Cash App RB", "0033A0", "Italian"),
    ];

    for (_, (team_name, color, nationality)) in team_info.iter().enumerate() {
        // Get drivers for this team - we need to clone because we need to own the data
        // to move it into the new Team struct
        let team_drivers: Vec<Driver> = drivers
            .iter()
            .filter(|d| d.team_name == *team_name)
            .cloned()
            .collect();

        // Find the car that matches the team name - need to clone to own the data
        let car = cars
            .iter()
            .find(|(name, _)| name == team_name)
            .map(|(_, car)| car.clone()) // Clone to get ownership
            .unwrap_or_else(|| Car {
                // Create a default car if needed
                name: "DefaultCar".to_string(),
                engine_power: 80,
                tyre_management: 80,
                reliability: 80,
                aerodynamics: 80,
            });

        // Build the team with the now owned data
        let team = Team {
            name: team_name.to_string(),
            team_color: color.to_string(),
            nationality: nationality.to_string(),
            drivers: team_drivers,
            car,
        };

        teams.push(team);
    }

    teams
}

pub fn simulate_practice_session(
    drivers: &Vec<Driver>,
    cars: &Vec<(String, Car)>,
    session_name: &str,
) -> Vec<RaceResult> {
    let mut rng = rand::rng();
    let mut results = Vec::new();

    for driver in drivers {
        let car = cars
            .iter()
            .find(|(name, _)| name == &driver.team_name)
            .map(|(_, car)| car) // Getting a reference to the car
            .unwrap(); // Unwrapping because we assume the car is found

        // Simulate session time based on skill, car attributes, and random variation
        let time_variation = rng.random_range(-1.5..1.5); // Random variation for practice times
        let driver_performance = (driver.skill_level as f32 * 0.4)
            + (car.engine_power as f32 * 0.3)
            + (car.tyre_management as f32 * 0.2)
            + (car.reliability as f32 * 0.1)
            + time_variation;

        let session_time = 90.0 - driver_performance; // 90 seconds is a baseline for the time

        results.push(RaceResult {
            driver_name: driver.full_name.clone(),
            team_name: driver.team_name.clone(),
            finish_position: 0, // Position will be determined later in qualification/race
            finish_time_seconds: session_time.round() as u8,
            dnf: false,
        });

        println!(
            "{} - {}: {} seconds (Session: {})",
            driver.full_name,
            driver.team_name,
            session_time.round(),
            session_name
        );
    }

    results
}

pub fn simulate_qualification(drivers: &Vec<Driver>, cars: &Vec<(String, Car)>) -> Vec<RaceResult> {
    let mut rng = rand::rng();
    let mut results = Vec::new();

    // Sorting drivers based on their performance and random qualifying factors
    for driver in drivers {
        let car = cars
            .iter()
            .find(|(name, _)| name == &driver.team_name)
            .map(|(_, car)| car) // Getting a reference to the car
            .unwrap(); // Unwrapping because we assume the car is found

        // Similar to the practice, but qualification might have less randomness and more focus on car performance
        let qualifying_time = (driver.skill_level as f32 * 0.5)
            + (car.engine_power as f32 * 0.4)
            + rng.random_range(-0.5..0.5);

        results.push(RaceResult {
            driver_name: driver.full_name.clone(),
            team_name: driver.team_name.clone(),
            finish_position: 0, // To be calculated after sorting
            finish_time_seconds: qualifying_time.round() as u8,
            dnf: false,
        });

        println!(
            "{} - {}: {} seconds (Qualification)",
            driver.full_name,
            driver.team_name,
            qualifying_time.round()
        );
    }

    // Sort by the qualifying time (lower time is better)
    results.sort_by(|a, b| a.finish_time_seconds.cmp(&b.finish_time_seconds));

    // Update positions after sorting
    for (position, result) in results.iter_mut().enumerate() {
        result.finish_position = (position + 1) as u8;
    }

    results
}

pub fn simulate_race(drivers: &Vec<Driver>, cars: &Vec<(String, Car)>) -> Vec<RaceResult> {
    let mut rng = rand::rng();
    let mut results = Vec::new();

    for driver in drivers {
        let car = cars
            .iter()
            .find(|(name, _)| name == &driver.team_name)
            .map(|(_, car)| car) // Getting a reference to the car
            .unwrap(); // Unwrapping because we assume the car is found

        // Simulate race performance considering skill, car performance, and some random race-related events
        let race_performance = (driver.skill_level as f32 * 0.4)
            + (car.engine_power as f32 * 0.3)
            + (car.tyre_management as f32 * 0.2)
            + (car.reliability as f32 * 0.1)
            + rng.random_range(-3.0..3.0); // Random race event variance

        let race_time = 120.0 - race_performance; // Race time based on performance

        results.push(RaceResult {
            driver_name: driver.full_name.clone(),
            team_name: driver.team_name.clone(),
            finish_position: 0, // Position will be determined after sorting
            finish_time_seconds: race_time.round() as u8,
            dnf: rng.random_bool(0.1), // 10% chance of DNF
        });

        println!(
            "{} - {}: {} seconds (Race)",
            driver.full_name,
            driver.team_name,
            race_time.round()
        );
    }

    // First sort by DNF status (non-DNF first), then by finish time
    results.sort_by(|a, b| {
        match (a.dnf, b.dnf) {
            // If both are DNF or both are not DNF, sort by time
            (true, true) | (false, false) => a.finish_time_seconds.cmp(&b.finish_time_seconds),
            // If a is DNF but b is not, b comes first
            (true, false) => std::cmp::Ordering::Greater,
            // If a is not DNF but b is, a comes first
            (false, true) => std::cmp::Ordering::Less,
        }
    });

    // Update positions after sorting
    for (position, result) in results.iter_mut().enumerate() {
        result.finish_position = (position + 1) as u8;
    }

    results
}

pub fn qualification_summary(results: &Vec<RaceResult>) {
    println!("\n🏁 Qualification Summary (Final Grid Order) 🏁");

    // Define the sections with their titles and corresponding driver ranges
    let sections = [
        ("--- Q3 Results ---", 0..10),
        ("--- Eliminated in Q2 ---", 10..15),
        ("--- Eliminated in Q1 ---", 15..results.len()),
    ];

    // Process each section
    for (title, range) in sections.iter() {
        println!("\n{}", title);

        // Get the drivers for this section
        let section_drivers = results
            .iter()
            .skip(range.start)
            .take(range.end - range.start);

        // Print each driver's info
        for result in section_drivers {
            println!(
                "P{}: {} | {} | Time: {}s{}",
                result.finish_position,
                result.driver_name,
                result.team_name,
                result.finish_time_seconds,
                if result.dnf { " | DNF" } else { "" }
            );
        }
    }
}

pub fn race_weekend_summary(results: &Vec<RaceResult>) {
    println!("\n🏁 Race Weekend Summary 🏁");

    for result in results {
        println!(
            "Driver: {} | Team: {} | Position: {} | Time: {}s | DNF: {}",
            result.driver_name,
            result.team_name,
            result.finish_position,
            result.finish_time_seconds,
            if result.dnf { "Yes" } else { "No" }
        );
    }
}
