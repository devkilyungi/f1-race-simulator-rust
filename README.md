# F1 Race Simulator

A Formula 1 race weekend simulator built in Rust. This project simulates drivers, teams, cars, and race events including practice sessions, qualifying, and the main race.

## Features

- Realistic driver attributes (skill, experience, aggression, consistency)
- Detailed car specifications (engine power, tyre management, reliability, aerodynamics)
- Team management with proper driver assignments
- Complete race weekend simulation:
  - Free practice sessions (FP1, FP2, FP3)
  - Qualifying session with grid positions
  - Full race simulation with finishing times and DNF possibilities
- Performance calculations based on driver skill and car capabilities
- Randomized elements to simulate real-world racing unpredictability

## Implementation Details

Built using Rust's ownership model and struct-based architecture to model the F1 ecosystem. The simulator leverages custom performance algorithms to create realistic race outcomes based on driver abilities and car specifications.

This project demonstrates practical applications of Rust concepts including:
- Structs and methods
- Ownership and borrowing
- Vectors and collections
- Random number generation
- Trait implementations
