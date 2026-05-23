# BioChain Indonesia

**BioChain Indonesia** - Blockchain-Based Biodiversity Observation System on Stellar

---

## Project Description

BioChain Indonesia is a decentralized biodiversity observation platform built on the Stellar blockchain using the Soroban SDK. The project enables researchers, conservation communities, students, and citizen scientists to securely record and manage species observations directly on-chain.

The smart contract stores biodiversity observation data such as:

- Species name
- Observation location
- Description
- Observer identity
- Verification status

By leveraging blockchain technology, BioChain ensures that biodiversity data is:

- Transparent
- Immutable
- Secure
- Traceable
- Tamper-resistant

This system can help support conservation efforts, scientific research, citizen science initiatives, and environmental transparency across Indonesia.

---

# Project Vision

Our vision is to create a decentralized biodiversity infrastructure for Indonesia and the global conservation community by:

- **Protecting Biodiversity Data**  
  Preventing manipulation and loss of ecological observation records.

- **Empowering Citizen Scientists**  
  Allowing anyone to contribute verified biodiversity observations.

- **Supporting Conservation Efforts**  
  Providing trusted on-chain ecological data for researchers and organizations.

- **Building Transparent Environmental Systems**  
  Creating auditable environmental records powered by blockchain.

- **Encouraging Community Participation**  
  Incentivizing biodiversity mapping through decentralized technology.

We envision a future where environmental data ownership belongs to the community and conservation efforts become more collaborative, transparent, and trustworthy.

---

# Key Features

## 1. Species Observation Recording

- Add biodiversity observations directly to the blockchain
- Store species name, location, observer, and descriptions
- Permanent decentralized data storage
- Automatic unique ID generation

---

## 2. Observation Verification

- Verify species observations
- Prevent fake or untrusted biodiversity reports
- Support expert validation workflows
- Improve data credibility for research purposes

---

## 3. Transparent Data Access

- Retrieve all biodiversity observations
- Publicly accessible blockchain records
- Easy frontend integration using Soroban SDK

---

## 4. Observation Deletion

- Remove observations by ID
- Efficient storage management
- Immediate blockchain state updates

---

## 5. Stellar & Soroban Integration

- Built using Soroban Smart Contracts
- Powered by the Stellar blockchain network
- Fast and low-cost transactions
- Scalable decentralized architecture

---

# Smart Contract Functions

## `add_observation()`

Add a new species observation to the blockchain.

### Parameters

- `species_name`
- `location`
- `description`
- `observer`

---

## `get_observations()`

Retrieve all stored biodiversity observations.

---

## `verify_observation()`

Verify a biodiversity observation by its ID.

### Parameters

- `id`

---

## `delete_observation()`

Delete an observation by ID.

### Parameters

- `id`

---

# Example Observation Data

| Field | Example |
|---|---|
| Species Name | Harimau Sumatra |
| Location | Taman Nasional Kerinci Seblat |
| Description | Observed near river area |
| Observer | Budi Santoso |
| Verified | true |

---

# Contract Structure

```rust
pub struct SpeciesObservation {
    id: u64,
    species_name: String,
    location: String,
    description: String,
    observer: String,
    verified: bool,
}
contract id:CB3ULMQLJ3PGL3X6MIYD3HBDDZBTVB4UBMAKVFLEUH7CTT6FNOKYMR6M