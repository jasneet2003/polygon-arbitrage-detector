# 🚀 Polygon Arbitrage Opportunity Detector Bot

A **simple, efficient, and fast bot** built in **Rust** that monitors decentralized exchanges (DEXes) on the **Polygon network** to detect potential arbitrage opportunities. It continuously checks token prices across multiple DEXes (like **QuickSwap** and **SushiSwap**) and simulates whether profitable trades are possible.

---

## Demo

*Follow this tutorial to set up bot!*

https://github.com/user-attachments/assets/493b878c-fbe2-4e11-af48-ad4783c95d4e



---

## Key Features

* **Multi-DEX Monitoring** — Fetches live token prices from **QuickSwap** and **SushiSwap** on Polygon.
* **Arbitrage Detection** — Compares prices and flags potential arbitrage opportunities.
* **Profit Simulation** — Estimates **net profit** after a configurable gas cost.
* **Database Logging** — Stores profitable opportunities in **SQLite** (`arbitrage_opportunities.db`).
* **Configurable** — All key parameters via a simple `config.toml`.
* **Asynchronous & Performant** — Built with **Rust** + **Tokio** for non-blocking I/O.

---

## Tech Stack

* **Language:** Rust
* **Blockchain:** `ethers-rs`
* **Async Runtime:** `Tokio`
* **Configuration:** `config-rs`
* **Database:** `rusqlite` (SQLite)
* **Network:** Polygon Mainnet

---

## Getting Started

Follow these steps to run the bot locally.

### 1. Prerequisites

* Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
* **Windows users:** Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

### 2. Clone the Repository

```bash
git clone https://github.com/jasneet2003/Polygon-Arbitrage-Opportunity-Detector-Bot.git
cd Polygon-Arbitrage-Opportunity-Detector-Bot
```

### 3. Configure the Bot

Inside the project directory, copy the example config file:

```bash
# Windows PowerShell
copy config.example.toml config.toml

# Linux/macOS
cp config.example.toml config.toml
```

Now edit `config.toml` and add your **Polygon RPC URL**.
→ Jump to [Configuration Details](#configuration-details) for help setting this up.

### 4. Run the Bot

```bash
cargo run
```

The bot will:

* Initialize
* Create `arbitrage_opportunities.db`
* Start monitoring opportunities 🚀

### 5. Stop the Bot

Press **Ctrl + C** in the terminal to safely stop it.

---

## Configuration Details

All settings are managed in `config.toml`.

### Polygon RPC URL (Required)

You’ll need a **personal RPC URL** (public RPCs are often rate-limited and unreliable).

**How to get one (free, \~2 minutes):**

1. Sign up at [Alchemy](https://www.alchemy.com/) or [Infura](https://infura.io/).
2. Create a new application/project.
3. Select **Polygon PoS (Mainnet)** as the network.
4. Copy your HTTPS URL (e.g. `https://polygon-mainnet.g.alchemy.com/v2/your-api-key`).
5. Paste it into the `rpc_url` field in `config.toml`.

### Other Configurable Parameters

* `amount_in` — Amount of base token (**WMATIC**) to use in the arbitrage simulation.
* `min_profit_threshold` — Minimum **net profit (USD)** required to log an opportunity.
* `simulated_gas_cost_usd` — Fixed **USD** estimate for the two swaps’ gas cost.

---

## License

This project is licensed under the **MIT License**.

---

## Notes

This bot is intended for **learning and experimentation**:

* It **does not execute trades automatically**; it only **detects** and **logs** opportunities.
* Real-world arbitrage is competitive and risky—proceed carefully if extending this for live trading.
