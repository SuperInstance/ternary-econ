# ternary-econ

Economic models with ternary market signals — portfolio optimization, supply/demand analysis, risk assessment, multi-agent market simulation, and trading strategies, all on {-1, 0, +1}.

## Why This Exists

Financial markets speak in three voices: **bullish**, **neutral**, and **bearish**. Traders reduce continuous price data to these ternary signals because they capture the essential information — direction and conviction — without the noise of precise numbers. This crate builds a complete economic modeling framework on ternary signals: market analysis, supply/demand dynamics, portfolio allocation, risk management, multi-agent simulation, and strategy evaluation.

The ternary constraint {-1, 0, +1} isn't a limitation — it's a discipline. Real traders think in signals, not precise returns. Analysts issue buy/hold/sell recommendations. Economists classify conditions as expansionary/neutral/contractionary. By formalizing this at the type level, the crate prevents the false precision that comes from treating estimated values as exact numbers, and it enables efficient simulation of large-scale agent-based markets.

This crate is part of the **Negative Space Intelligence** ecosystem.

## Core Concepts

- **MarketSignal** — A ternary market direction: `Bearish` (-1), `Neutral` (0), or `Bullish` (+1).
- **Asset** — A financial asset with name, signal, volatility (0–1), and expected return (-1 to +1).
- **TernaryMarket** — A collection of assets with aggregate signal (majority voting), average return, and average volatility.
- **SupplyDemand** — Supply/demand model with ternary indicators. Computes price pressure, updates prices, and measures imbalance.
- **PortfolioOptimizer** — Allocates portfolio weights based on risk level and asset signals. Three risk profiles: `Avoid`, `Neutral`, `Embrace`.
- **RiskAssessment** — Evaluates portfolios against risk profiles, returning `Accept`, `Caution`, or `Reject` verdicts.
- **TernaryAgent** — A trading agent with risk tolerance, cash, holdings, and signal generation. Executes buy/sell/hold based on market signals.
- **MarketSimulation** — Multi-agent simulation with price discovery driven by aggregate buying/selling pressure.
- **Strategies** — Built-in `momentum_strategy`, `contrarian_strategy`, and `buy_and_hold_strategy`.
- **Sharpe Ratio** — Risk-adjusted return metric for simulation results.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-econ = "0.1"
```

```rust
use ternary_econ::*;

// Create assets with ternary signals
let assets = vec![
    Asset::new("TECH", MarketSignal::Bullish, 0.2, 0.08),
    Asset::new("BONDS", MarketSignal::Neutral, 0.05, 0.02),
    Asset::new("COMMOD", MarketSignal::Bearish, 0.3, -0.05),
];

// Market analysis
let market = TernaryMarket::new(assets.clone());
assert_eq!(market.overall_signal, MarketSignal::Neutral);
println!("Avg return: {:.2}%", market.average_return() * 100.0);
println!("Avg volatility: {:.2}%", market.average_volatility() * 100.0);

// Portfolio optimization
let optimizer = PortfolioOptimizer::new(RiskLevel::Avoid);
let weights = optimizer.allocate(&assets);
for (name, w) in &weights {
    println!("{}: {:.1}%", name, w * 100.0);
}

// Risk assessment
let risk = RiskAssessment::new(RiskLevel::Avoid, 0.5);
match risk.assess(&assets) {
    RiskVerdict::Accept => println!("Portfolio accepted"),
    RiskVerdict::Caution(msg) => println!("⚠️ {}", msg),
    RiskVerdict::Reject(msg) => println!("❌ {}", msg),
}

// Supply and demand
let mut sd = SupplyDemand::new(MarketSignal::Bearish, MarketSignal::Bullish, 100.0);
assert_eq!(sd.price_pressure(), MarketSignal::Bullish); // high demand
let new_price = sd.update_price();
assert!(new_price > 100.0);

// Multi-agent simulation
let agents = vec![
    TernaryAgent::new(1, RiskLevel::Embrace, 1000.0),
    TernaryAgent::new(2, RiskLevel::Neutral, 2000.0),
    TernaryAgent::new(3, RiskLevel::Avoid, 1500.0),
];
let mut sim = MarketSimulation::new(agents, 100.0);
sim.run(&[
    MarketSignal::Bullish, MarketSignal::Bullish,
    MarketSignal::Neutral, MarketSignal::Bearish,
]);
println!("Final price: {:.2}", sim.final_price());
println!("Return: {:.2}%", sim.total_return() * 100.0);
println!("Volatility: {:.2}%", sim.volatility() * 100.0);
println!("Sharpe: {:.2}", sharpe_ratio(&sim, 0.02));

// Trading strategies
let signals = vec![MarketSignal::Bullish, MarketSignal::Bearish, MarketSignal::Neutral];
let momentum = momentum_strategy(&signals);
let contrarian = contrarian_strategy(&signals);
```

## API Overview

### Market Types
| Type | Description |
|---|---|
| `MarketSignal` | `Bearish` / `Neutral` / `Bullish` |
| `Asset` | Name + signal + volatility + expected return |
| `TernaryMarket` | Asset collection with aggregate metrics |
| `SupplyDemand` | Price dynamics from supply/demand signals |
| `RiskLevel` | `Avoid` / `Neutral` / `Embrace` |
| `RiskVerdict` | `Accept` / `Caution(msg)` / `Reject(msg)` |

### Portfolio & Risk
| Type | Description |
|---|---|
| `PortfolioOptimizer` | Risk-aware weight allocation |
| `RiskAssessment` | Portfolio evaluation against risk profile |

### Agents & Simulation
| Type | Description |
|---|---|
| `TernaryAgent` | Trading agent with cash, holdings, and signal |
| `TradeAction` | `Buy(n)` / `Sell(n)` / `Hold` |
| `MarketSimulation` | Multi-agent market with price discovery |

### Strategies & Metrics
| Function | Description |
|---|---|
| `momentum_strategy(signals)` | Follow the trend |
| `contrarian_strategy(signals)` | Oppose the trend |
| `buy_and_hold_strategy(len)` | Buy once, hold |
| `aggregate_history(signals)` | Majority signal from history |
| `sharpe_ratio(sim, risk_free)` | Risk-adjusted return |

## How It Works

Market signals aggregate via majority voting: sum all `Bearish` (-1), `Neutral` (0), and `Bullish` (+1) signals and take the sign of the result. This produces a robust aggregate that's resistant to noise — a single outlier signal can't flip the market direction unless it's already balanced.

Portfolio allocation scores each asset by combining its signal (mapped to 0/0.5/1.0), expected return, and a risk adjustment based on volatility and the investor's risk level. Risk-averse investors get heavy penalties for volatile assets; risk-seeking investors barely notice. Scores are normalized to sum to 1.0, producing portfolio weights.

The market simulation runs agents in parallel each step. Every agent generates a signal from the external market data (adjusted by their risk profile), then executes a trade. Aggregate buying pressure (total buy shares minus sell shares) drives price changes through a sensitivity factor. This creates emergent market dynamics: bullish herds push prices up, bearish panics drive them down, and risk-averse agents provide stabilizing counterweight.

Risk assessment uses tiered thresholds: `Avoid` rejects anything above the specified volatility with negative returns; `Neutral` allows more slack; `Embrace` only flags extreme cases. The three-valued verdict (`Accept`/`Caution`/`Reject`) maps naturally to ternary logic.

## Use Cases

1. **Agent-based market simulation** — Study how markets behave when heterogeneous agents (risk-seeking, risk-neutral, risk-averse) interact. The ternary signal model produces realistic boom-bust cycles from simple local rules.

2. **Portfolio construction** — Build ternary-signal-driven portfolios with risk-aware allocation. The `PortfolioOptimizer` handles the signal-to-weight conversion, respecting the investor's risk tolerance.

3. **Trading strategy backtesting** — Compare `momentum_strategy`, `contrarian_strategy`, and `buy_and_hold_strategy` across signal sequences. The `MarketSimulation` provides a consistent execution environment with `sharpe_ratio` for evaluation.

4. **Economic indicator analysis** — Reduce complex economic data to ternary signals (contractionary/neutral/expansionary) and analyze aggregate trends. The `aggregate_history` function computes the net directional bias.

## Ecosystem

| Crate | Relationship |
|---|---|
| `ternary-bayesian` | Bayesian updates for market belief revision |
| `ternary-network` | Agent interaction networks and systemic risk |
| `ternary-attention` | Attention over market signal sequences |
| `ternary-energy` | Capital conservation and thermodynamic analogies |
| `ternary-logic` | Formal reasoning about market conditions |

## License

MIT
