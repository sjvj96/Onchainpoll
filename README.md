# OnChainPoll - Soroban Smart Contract

## 📌 Description

OnChainPoll is a decentralized polling application built using Soroban smart contracts on the Stellar blockchain.
It allows users to create polls, vote securely, and view results transparently on-chain.

Each voter can only vote once, ensuring fairness and preventing duplicate voting without relying on a central authority.

---

## 🚀 Features

* Create a poll with 3 options
* Vote on a poll (one vote per user)
* Prevent duplicate voting
* View real-time poll results
* Close poll to stop voting
* View list of voters and their selected options

---

## 🛠️ Technologies

* **Language**: Rust (`no_std`)
* **Framework**: Soroban SDK
* **Blockchain**: Stellar (Testnet)

---

## 📄 Smart Contract ID (Testnet)

```
CD3GGHOSDJXNR5VL367RX5D27VAWEST6DXXV47OYH7RSBSKYGFRU44U3
```

---

## 🖼️ Screenshots

(Add your screenshot here)

Example:

* Create poll success
* Vote success
* Get results

```
screenshot.png
```

---

## 📦 How It Works

1. User creates a poll with a question and 3 options
2. The poll is stored on the blockchain
3. Users vote using a unique identifier (address/string)
4. The smart contract ensures each user can only vote once
5. Results can be retrieved anytime
6. Poll can be closed to stop further voting

---

## 🧪 Smart Contract Functions

| Function                                              | Description                     |
| ----------------------------------------------------- | ------------------------------- |
| `create_poll(question, option_a, option_b, option_c)` | Create a new poll               |
| `vote(voter_address, option_id)`                      | Vote for an option (0, 1, or 2) |
| `get_results()`                                       | Get poll results                |
| `close_poll()`                                        | Close the poll                  |
| `get_voters()`                                        | Get list of voters              |

---

## 🎯 Use Case

* Online voting system
* Community decision making
* Simple DAO governance simulation
* Educational blockchain demo

---
Explorer:
https://lab.stellar.org/r/testnet/contract/CD3GGHOSDJXNR5VL367RX5D27VAWEST6DXXV47OYH7RSBSKYGFRU44U3


## 👨‍💻 Author

Your Name

---

## 📜 License

MIT License
