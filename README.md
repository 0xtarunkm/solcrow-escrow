# Solcrow Escrow Program

This decentralized application allows users to post job listings for work they need done, and freelancers can select these jobs, complete them, and get paid in a secure, trustless manner.

## Features

- **Create Listing**: Users can create job listings with details about the work they need done.
- **Browse Listings**: Freelancers can browse through available listings to find work they want to complete.
- **Select Listing**: Freelancers can choose a job to work on from the marketplace.

## Table of Contents

- [Solcrow Escrow Program](#solcrow-escrow-program)
  - [Features](#features)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Marketplace Program](#marketplace-program)
    - [Web Interface](#web-interface)

## Installation

To install and build the project locally:

1. Clone the repository:

   ```bash
   git clone https://github.com/0xtarunkm/solwork-escrow.git
   cd solwork-escrow
   ```

2. Install dependencies:

   ```bash
   yarn install
   ```

3. Build the Anchor program:

   ```bash
   anchor build
   ```

4. Deploy the program to Solana Localnet:

   ```bash
   anchor deploy
   ```

## Usage

Once deployed, users can interact with the contract via CLI or through a web interface (if implemented). The contract supports the following operations:

1. **Create Escrow**: Create a new escrow for a specific listing.
2. **Deposit money in vault**: Maker can deposit money in vault
3. **Refund the money**: Maker can refund the money to itself from the vault
4. **Settle the listing**: Taker can settle the listing and withdraw money

### Marketplace Program

The marketplace program for solwork can be found at [marketplace program](https://github.com/0xtarunkm/solwork-marketplace.git)

### Web Interface

The web interface for this solwork can be found at [solwork web](https://github.com/0xtarunkm/solwork.git)
