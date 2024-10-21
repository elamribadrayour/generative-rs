# Generative Algorithms in Rust

Generative algorithms implemented in Rust. This repository explores genetic algorithms and neural networks, using Rust programming language and Nannou for visualization.

## Table of Contents

- [About](#about)
- [Project Structure](#project-structure)
- [Setup and Installation](#setup-and-installation)
- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)

## About

This project is inspired by the blog series "[Learning to Fly](https://pwy.io/posts/learning-to-fly-pt4/)" and serves as a learning platform for implementing generative algorithms in Rust. It combines genetic algorithms with neural networks to simulate evolutionary processes and neural computations.

## Project Structure

```plaintext
.
├── Cargo.lock
├── Cargo.toml
├── libs
│   ├── genetic-algorithm
│   └── neural-network
├── readme.md
└── ui
```

### Libraries

- **Genetic Algorithm**: Contains modules like `chromosome`, `crossover`, `mutation`, and `selection`.
- **Neural Network**: Implements neural network components such as `layer`, `network`, and `neuron`.

### User Interface

- Built with Nannou, the UI simulates entities like `animal`, `food`, and `world`.

## Setup and Installation

To get started, ensure you have Rust and Cargo installed. Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/yourproject.git
cd yourproject
cargo build
```

## Usage

Run the simulation using:

```bash
cargo run --bin ui
```

This will start the visual simulation, allowing you to witness the generative algorithms in action.

## Features

- Modular design for genetic algorithms.
- Flexible neural network implementation.
- Interactive UI with Nannou for visualization.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [license](license) file for details.
