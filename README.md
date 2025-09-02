🚀 Polygon Arbitrage Opportunity Detector Bot

A simple, efficient, and fast bot built in Rust that monitors decentralized exchanges (DEXes) on the Polygon network to detect potential arbitrage opportunities.

It continuously checks token prices across multiple DEXes (like QuickSwap and SushiSwap) and simulates whether profitable arbitrage trades are possible.

🎥 Demo

👉 A short 20–30 second screen recording is the best way to demonstrate your bot in action.
You can use free tools like OBS Studio or the built-in Windows Game Bar (Win + G) to capture your terminal.

✨ Key Features

🔄 Multi-DEX Monitoring – Fetches live token prices from QuickSwap and SushiSwap on Polygon.

📈 Arbitrage Detection – Compares prices and identifies arbitrage opportunities.

💰 Profit Simulation – Calculates estimated net profit after accounting for gas fees.

🗂️ Database Logging – Saves all profitable opportunities in a local SQLite database (arbitrage_opportunities.db).

⚙️ Fully Configurable – Adjust all parameters easily via a config.toml file.

⚡ Blazing Fast & Async – Built with Rust + Tokio for non-blocking, efficient performance.

🛠️ Tech Stack

Language: Rust

Blockchain Interaction: ethers-rs

Async Runtime: Tokio

Config Handling: config-rs

Database: rusqlite (SQLite)

Network: Polygon Mainnet

🚀 Getting Started

Follow these steps to run the bot locally:

1. Prerequisites

Install the Rust toolchain
.

Windows users: Install Microsoft C++ Build Tools
.

2. Clone the Repository
git clone https://github.com/jasneet2003/Polygon-Arbitrage-Opportunity-Detector-Bot.git
cd Polygon-Arbitrage-Opportunity-Detector-Bot

3. Configure the Bot

Inside the project directory, copy the example config file:

# Windows PowerShell
copy config.example.toml config.toml

# Linux/macOS
cp config.example.toml config.toml


Now edit config.toml and add your Polygon RPC URL.

👉 Jump to Configuration Details
 for help setting this up.

4. Run the Bot
cargo run


The bot will:

Initialize

Create arbitrage_opportunities.db

Start monitoring opportunities 🚀

5. Stop the Bot

Press Ctrl + C in the terminal to safely stop it.

⚙️ Configuration Details

All settings are managed in config.toml.

🔗 Polygon RPC URL (Required)

You’ll need a personal RPC URL (public ones are unreliable & rate-limited).

How to get one (free, 2 minutes):

Sign up at Alchemy
 or Infura
.

Create a new app/project.

Select Polygon PoS (Mainnet) as the network.

Copy your HTTPS URL (e.g. https://polygon-mainnet.g.alchemy.com/v2/your-api-key).

Paste it into the rpc_url field in config.toml.

⚙️ Other Configurable Parameters

amount_in – Amount of base token (WMATIC) to simulate arbitrage with.

min_profit_threshold – Minimum net profit (USD) required before logging an opportunity.

simulated_gas_cost_usd – Estimated gas fees (USD) for the two required swaps.

📜 License

This project is licensed under the MIT License – feel free to use, modify, and share.

🌟 Final Notes

This bot is a learning + experimental project.

It does not execute trades automatically (just detects & logs).

Always be careful when adapting it to real trading — arbitrage is competitive and risky.
