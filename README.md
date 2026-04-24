# Whistleblower Reward Smart Contract

## Project Description

The **Whistleblower Reward Smart Contract** is a decentralized reward system built on **Soroban**, Stellar’s smart contract platform.  
It enables organizations to reward anonymous whistleblowers after a reported case has been reviewed and successfully resolved.

The contract ensures transparency in reward distribution while preserving the anonymity of the whistleblower through blockchain wallet addresses.

---

## What it does

This smart contract manages whistleblower reports in three stages:

1. **Submit Report**  
   A whistleblower anonymously submits a report with a reward amount attached.

2. **Resolve Report**  
   An authorized admin reviews the report and marks it as resolved.

3. **Release Reward**  
   Once resolved, the admin releases the reward, and the report status changes to rewarded.

4. **Track Reports**  
   Report details and statuses can be viewed on-chain for accountability and transparency.

---

## Features

- **Anonymous Reporting**  
  Whistleblowers interact through wallet addresses without exposing identity.

- **Resolution-Based Rewards**  
  Rewards are only released after admin approval and case resolution.

- **Transparent Workflow**  
  Every report follows a visible lifecycle:
  `Submitted → Resolved → Rewarded`

- **Admin Authorization**  
  Only the authorized admin can resolve reports and release rewards.

- **On-Chain Record Keeping**  
  All report data is stored securely on Soroban smart contract storage.

---

## Smart Contract Functions

### `initialize(admin)`
Initializes the contract and sets the admin address.

### `submit_report(reporter, reward)`
Allows a whistleblower to submit a report with a reward value.

### `resolve_report(admin, id)`
Allows the admin to mark a report as resolved.

### `release_reward(admin, id)`
Allows the admin to release the reward after resolution.

### `get_report(id)`
Returns the report details, reward amount, and current status.

---

## Tech Stack

- **Rust**
- **Soroban SDK**
- **Stellar Blockchain**
- **Stellar IDE**

---

## Use Cases

- Corporate whistleblower protection systems
- Fraud reporting platforms
- Anti-corruption reward systems
- Governance and compliance reporting

---

## Future Improvements

- Integrate **Soroban token transfers** for real reward payouts
- Add **encrypted report metadata**
- Implement **multi-admin verification**
- Add **severity-based reward tiers**
- Build a **frontend dashboard** for reporters and administrators

---

## Current Status

This is an **MVP (Minimum Viable Product)** version of the contract.

Currently:
- reports can be submitted,
- admins can resolve them,
- rewards can be marked as released.

Future versions will support **actual token payouts** directly to the whistleblower wallet using Soroban token contracts.

---

Wallet address: GDIA4VFCC6NVYZQUZSF44QV2XYRS6IGL73FBUFADR2HQJUSDO5M7EQGF

Contract address: CCBDSLCVKEZVEQ5S5KOPZEAXISRSD7WDIXK3G2J43WJV4MNSFOWL3Q5Y

https://stellar.expert/explorer/testnet/contract/CCBDSLCVKEZVEQ5S5KOPZEAXISRSD7WDIXK3G2J43WJV4MNSFOWL3Q5Y

<img width="1863" height="977" alt="image" src="https://github.com/user-attachments/assets/045a0366-5636-49a3-850e-3b79ae2561bf" />
