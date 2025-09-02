# Polygon Arbitrage Opportunity Detector Bot

A simple and efficient bot built in Rust that detects potential arbitrage opportunities on the Polygon network by monitoring prices across multiple decentralized exchanges (DEXes).

---

###  Demo

*(Here, you can record your screen for 20-30 seconds showing the bot running in the terminal and then upload it as a GIF. This is highly recommended and very impressive.)*

![Demo GIF placeholder](https://via.placeholder.com/800x400.png?text=Record+a+GIF+of+the+bot+running+and+place+it+here)

---

### Key Features

- **Multi-DEX Monitoring:** Continuously fetches and compares token prices from QuickSwap and SushiSwap on the Polygon Mainnet.
- **Arbitrage Detection:** Identifies potential arbitrage opportunities by calculating the price difference for a given trade size.
- **Simulated Profit Calculation:** Calculates the estimated *net profit* after accounting for a simplified, configurable gas cost.
- **Database Logging:** Logs all profitable opportunities to a local SQLite database (`arbitrage_opportunities.db`) for persistent record-keeping.
- **Configurable:** All key parameters (RPC URL, token/DEX addresses, trade size, profit thresholds) are managed in a simple `config.toml` file.
- **Asynchronous & Performant:** Built with Rust and Tokio for efficient, non-blocking network requests.

### Technology Stack

- **Language:** Rust
- **Blockchain Interaction:** `ethers-rs`
- **Asynchronous Runtime:** `Tokio`
- **Configuration:** `config-rs`
- **Database:** `rusqlite` (SQLite)
- **Network:** Polygon Mainnet

---

### Setup and Usage

**1. Prerequisites:**
   - [Rust](https://www.rust-lang.org/tools/install) toolchain installed.
   - For Windows, [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) are required.

**2. Clone the Repository:**
   ```bash
   git clone <your-repo-url>
   cd arbitrage_bot