# Ultimate Texas Holdem
This repository contains code that leverages the pyo3 crate from Rust which compiles into native python code. 
The core engine of the Ultimate Texas Holdem game and all its properties such as the Players, Cards, Game Logic and Wagering system is designed and implemented in Rust. The pure python code contains a wrapper layer which allows for the actual interaction with the console / user.

## Phases
- [ ] Create a Command Line Interface Version of the Game
- [ ] Wrap CLI into GUI
- [ ] Send Logs as unstrcutred data to MongoDB Cloud Atlas Database
- [ ] Containerize (w/ Docker)
- [ ] Deploy on AWS Kubernetes Cluster
