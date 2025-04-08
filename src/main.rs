mod utils;
mod models;

use utils::*;

fn main() {
    let drivers: Vec<models::Driver> = create_drivers();
    let cars: Vec<(String, models::Car)> = create_cars();
    let teams: Vec<models::Team> = create_teams(&drivers, &cars);

    println!();
    println!("{:#?}", teams);
    println!();

    // Simulate Free Practice sessions
    println!("\n🏁 Free Practice 1 Summary 🏁");
    let fp1_results = simulate_practice_session(&drivers, &cars, "FP1");

    // println!();
    // println!("{:#?}", fp1_results);
    // println!();

    println!("\n🏁 Free Practice 2 Summary 🏁");
    let fp2_results = simulate_practice_session(&drivers, &cars, "FP2");

    // println!();
    // println!("{:#?}", fp2_results);
    // println!();

    println!("\n🏁 Free Practice 3 Summary 🏁");
    let fp3_results = simulate_practice_session(&drivers, &cars, "FP3");

    // println!();
    // println!("{:#?}", fp3_results);
    // println!();

    // Simulate Qualification
    println!("\n🏁 Qualification Summary 🏁");
    let qualification_results = simulate_qualification(&drivers, &cars);

    // println!();
    // println!("{:#?}", qualification_results);
    // println!();

    // Qualification summary in order
    qualification_summary(&qualification_results);

    // Simulate Race Day
    println!("\n🏁 Race Results Summary 🏁");
    let race_results = simulate_race(&drivers, &cars);

    // Give Race Weekend Summary
    race_weekend_summary(&race_results);
}