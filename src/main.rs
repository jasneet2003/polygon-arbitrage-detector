use ethers::prelude::*;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use rusqlite::{Connection, params}; // Import for SQLite

// --- ABIs (as before) ---
abigen!(
    IUniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)
    ]"#,
);

// --- Configuration Structs (as before) ---
#[derive(Debug, Deserialize)]
struct Config {
    network: NetworkConfig,
    settings: SettingsConfig,
    simulation: SimulationConfig,
    tokens: HashMap<String, Address>,
    dexes: HashMap<String, Address>,
}

#[derive(Debug, Deserialize)]
struct NetworkConfig {
    rpc_url: String,
}

#[derive(Debug, Deserialize)]
struct SettingsConfig {
    amount_in: f64,
    min_profit_threshold: f64,
}

#[derive(Debug, Deserialize)]
struct SimulationConfig {
    simulated_gas_cost_usd: f64,
}

// NEW: Function to setup the database and create the table
fn setup_database() -> Result<Connection> {
    let conn = Connection::open("arbitrage_opportunities.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            buy_dex TEXT NOT NULL,
            sell_dex TEXT NOT NULL,
            amount_in REAL NOT NULL,
            gross_profit_usd REAL NOT NULL,
            net_profit_usd REAL NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}


#[tokio::main]
async fn main() -> Result<()> {
    // --- 1. Load Configuration & Setup Database ---
    let builder = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    let config: Config = builder.try_deserialize()?;
    
    // NEW: Setup the database connection
    let db_conn = setup_database()?;

    println!("--- Polygon Arbitrage Detector Initialized ---");
    println!("Database connection established. Logging to 'arbitrage_opportunities.db'");
    
    // ... (rest of the initialization prints)

    // --- (Setup is the same as before) ---
    let provider = Provider::<Http>::try_from(config.network.rpc_url)?;
    let client = Arc::new(provider);
    let wmatic_address = config.tokens["WMATIC"];
    let usdc_address = config.tokens["USDC"];
    let quickswap_router_address = config.dexes["QuickSwap"];
    let sushiswap_router_address = config.dexes["SushiSwap"];
    let amount_in = ethers::utils::parse_ether(config.settings.amount_in)?;
    let path: Vec<Address> = vec![wmatic_address, usdc_address];
    let quickswap_router = IUniswapV2Router::new(quickswap_router_address, client.clone());
    let sushiswap_router = IUniswapV2Router::new(sushiswap_router_address, client.clone());

    // --- 5. Main Loop ---
    loop {
        let timestamp = chrono::Local::now();
        println!("\n----------------------------------------");
        println!("Checking prices at {}...", timestamp.to_rfc2822());

        let amounts_out_quickswap = quickswap_router.get_amounts_out(amount_in, path.clone()).call().await?;
        let amount_out_quickswap = amounts_out_quickswap[1];
        let price_quickswap = ethers::utils::format_units(amount_out_quickswap, 6)?.parse::<f64>()?;
        
        let amounts_out_sushiswap = sushiswap_router.get_amounts_out(amount_in, path.clone()).call().await?;
        let amount_out_sushiswap = amounts_out_sushiswap[1];
        let price_sushiswap = ethers::utils::format_units(amount_out_sushiswap, 6)?.parse::<f64>()?;

        println!("   - QuickSwap price: {:.5} USDC", price_quickswap);
        println!("   - SushiSwap price: {:.5} USDC", price_sushiswap);

        // --- 6. Arbitrage Logic ---
        let (buy_dex, sell_dex, gross_profit) = if price_quickswap > price_sushiswap {
            ("SushiSwap", "QuickSwap", price_quickswap - price_sushiswap)
        } else {
            ("QuickSwap", "SushiSwap", price_sushiswap - price_quickswap)
        };
        
        let net_profit = gross_profit - config.simulation.simulated_gas_cost_usd;

        if net_profit > config.settings.min_profit_threshold {
            println!("\n🔥🔥🔥 ARBITRAGE OPPORTUNITY DETECTED! 🔥🔥🔥");
            println!("   - BUY on {}, SELL on {}", buy_dex, sell_dex);
            println!("   - Gross Profit: ${:.5}", gross_profit);
            println!("   - Est. Gas Cost: ${:.2}", config.simulation.simulated_gas_cost_usd);
            println!("   - NET PROFIT: ${:.5}", net_profit);
            
            // NEW: Log the finding to the SQLite database
            db_conn.execute(
                "INSERT INTO opportunities (timestamp, buy_dex, sell_dex, amount_in, gross_profit_usd, net_profit_usd) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![timestamp.to_rfc3339(), buy_dex, sell_dex, config.settings.amount_in, gross_profit, net_profit],
            )?;
            println!("   - ✅ Opportunity logged to database.");
        } else {
            println!("   - No profitable opportunity found.");
        }

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}