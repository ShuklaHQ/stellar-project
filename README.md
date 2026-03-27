# Travel Booking DApp on Stellar (Soroban)

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/d0539dc0-7ee4-49f3-8af6-722bcede2055" />

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/9410f03f-4457-42c3-b03d-c75ab4ed2dee" />

## 📌 Project Description
A decentralized travel booking application built on **Stellar Soroban smart contracts**. It allows users to book, view, and cancel travel reservations directly on-chain, ensuring transparency, immutability, and security.

---

## 🚀 What It Does
- Users can **book trips** by specifying destination and date.
- Retrieve existing bookings stored on-chain.
- Cancel bookings securely without intermediaries.
- All bookings are stored in Stellar’s decentralized ledger.

---

## ✨ Features
- **Decentralized**: No central authority, bookings are immutable and transparent.
- **Secure**: Uses Stellar Soroban smart contracts for trustless execution.
- **Simple API**: Functions for booking, retrieving, and canceling trips.
- **Extendable**: Can be expanded to include payments, loyalty points, or group bookings.

---

## 🔗 Deployed Smart Contract
- **Contract Address**: `CBLY4BHLFJ2YRKD4J2TCAC56SKQO5SNR6FWCHMTB2SDL3723BPNHVSO6`  
- **Network**: Stellar Testnet (Soroban)

👉 You can interact with the contract using Soroban CLI or integrate it into a frontend DApp.

---

## 🛠️ Future Enhancements
- Integration with **Stellar USDC** for payments.
- Support for **multi-leg journeys**.
- Loyalty rewards system using **custom tokens**.
- Frontend DApp with wallet integration (e.g., Freighter).

---

## 📚 Getting Started
1. Clone the repository.
2. Install Rust and Soroban CLI.
3. Build the contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
