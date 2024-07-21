# Solana Token dApp

This repository contains a Solana dApp project for creating and managing custom tokens on the Solana blockchain. The project includes smart contract code written in Rust using the Anchor framework, along with deployment scripts and configuration files.

## Getting Started

Follow these instructions to set up and deploy the project.

### Prerequisites

- Rust
- Solana CLI
- Anchor CLI
- Node.js and npm

### Installation

1. **Clone the repository:**

   git clone https://github.com/balu6914/sol-token.git
   cd sol-token
Install dependencies:
npm install

Build the program:
anchor build

Deploy the program:

solana config set --url https://api.devnet.solana.com
anchor deploy

Usage
After deploying the program, you can interact with it using the client script or by integrating it with a front-end application.
