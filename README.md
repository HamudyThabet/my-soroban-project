# TerraTrust

### One-Line Description
A decentralised agricultural land registry built on Soroban to enable instant title verification and asset coordination for rural farmers.

### Problem & Solution
* **Problem:** Juan, a smallholder rice farmer in Baliwag, Philippines, faces a 12-month delay and $500 in predatory brokerage/legal fees to verify and transfer a land title when trying to secure an agricultural production loan, causing him to miss the planting window and lose 40% of his annual crop yield.
* **Solution:** Juan instantly registers and transfers a digitized land parcel token through the TerraTrust web app using a Soroban smart contract that updates land ownership records, eliminating intermediary delays and reducing title verification costs to less than $0.01 per transaction.

### Timeline
* **Week 1:** Core Soroban Smart Contract Architecture, Data Types, and Unit Testing Framework.
* **Week 2:** Web App Interface integration using Freighter wallet for authenticating property handovers.

### Stellar Features Used
* Soroban Smart Contracts (State Management and Ownership validation)
* XLM native token tracking for ultra-low computational gas fees.

### Vision and Purpose
TerraTrust unlocks localized rural liquidity by removing bureaucratic friction, turning unmapped geographical land tracts into formal, programmable digital equity assets directly integrated within decentralized finance tracks.

### Prerequisites
* Rust (v1.75+)
* Soroban CLI (v21.0.0+)
* Target `wasm32-unknown-unknown` installed

### How to Build
```bash
soroban contract build
```

### Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CDJI4AOB3OVYIJD7VSOYKJ3SWP6Z5YQYWB7PWAZ2N55OAP7KWFIXDQCO` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CDJI4AOB3OVYIJD7VSOYKJ3SWP6Z5YQYWB7PWAZ2N55OAP7KWFIXDQCO) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/3502402b7b04705617f4af15e1d0612f28d80a815cfa93449eb04c1adef10c51) |
| Deployed | 2026-06-26 08:06:19 UTC |
| Wallet | freighter (`GC7N…X57N`) |
![Screenshot 2026-06-26 160725_1.png](./assets/Screenshot%202026-06-26%20160725_1.png)
