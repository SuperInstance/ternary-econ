#![forbid(unsafe_code)]

//! Economic models with ternary market signals.
//!
//! Provides ternary market analysis, supply/demand with ternary indicators,
//! portfolio optimization, risk assessment, market simulation with ternary
//! agents, and trading strategies.

/// Ternary market signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MarketSignal {
    Bearish,
    Neutral,
    Bullish,
}

impl MarketSignal {
    pub fn to_i8(self) -> i8 {
        match self {
            MarketSignal::Bearish => -1,
            MarketSignal::Neutral => 0,
            MarketSignal::Bullish => 1,
        }
    }

    pub fn from_i8(v: i8) -> Self {
        match v {
            -1 => MarketSignal::Bearish,
            0 => MarketSignal::Neutral,
            _ => MarketSignal::Bullish,
        }
    }

    pub fn all() -> &'static [MarketSignal; 3] {
        &[MarketSignal::Bearish, MarketSignal::Neutral, MarketSignal::Bullish]
    }
}

/// Risk assessment level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    Avoid,
    Neutral,
    Embrace,
}

impl RiskLevel {
    pub fn weight(self) -> f64 {
        match self {
            RiskLevel::Avoid => 0.2,
            RiskLevel::Neutral => 0.5,
            RiskLevel::Embrace => 0.8,
        }
    }
}

/// An asset in the portfolio.
#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub signal: MarketSignal,
    pub volatility: f64, // 0.0 to 1.0
    pub expected_return: f64, // -1.0 to 1.0
}

impl Asset {
    pub fn new(name: &str, signal: MarketSignal, volatility: f64, expected_return: f64) -> Self {
        Asset {
            name: name.to_string(),
            signal,
            volatility: volatility.clamp(0.0, 1.0),
            expected_return: expected_return.clamp(-1.0, 1.0),
        }
    }
}

/// Ternary market with signal aggregation.
#[derive(Clone, Debug)]
pub struct TernaryMarket {
    pub assets: Vec<Asset>,
    pub overall_signal: MarketSignal,
}

impl TernaryMarket {
    pub fn new(assets: Vec<Asset>) -> Self {
        let overall = Self::aggregate_signal(&assets);
        TernaryMarket { assets, overall_signal: overall }
    }

    /// Aggregate signals using majority voting.
    pub fn aggregate_signal(assets: &[Asset]) -> MarketSignal {
        if assets.is_empty() {
            return MarketSignal::Neutral;
        }
        let sum: i8 = assets.iter().map(|a| a.signal.to_i8()).sum();
        if sum > 0 { MarketSignal::Bullish }
        else if sum < 0 { MarketSignal::Bearish }
        else { MarketSignal::Neutral }
    }

    /// Average expected return across assets.
    pub fn average_return(&self) -> f64 {
        if self.assets.is_empty() { return 0.0; }
        self.assets.iter().map(|a| a.expected_return).sum::<f64>() / self.assets.len() as f64
    }

    /// Average volatility across assets.
    pub fn average_volatility(&self) -> f64 {
        if self.assets.is_empty() { return 0.0; }
        self.assets.iter().map(|a| a.volatility).sum::<f64>() / self.assets.len() as f64
    }
}

/// Supply and demand with ternary indicators.
#[derive(Clone, Debug)]
pub struct SupplyDemand {
    pub supply_signal: MarketSignal,
    pub demand_signal: MarketSignal,
    pub price: f64,
}

impl SupplyDemand {
    pub fn new(supply: MarketSignal, demand: MarketSignal, price: f64) -> Self {
        SupplyDemand {
            supply_signal: supply,
            demand_signal: demand,
            price,
        }
    }

    /// Determine price pressure: bullish if demand > supply, etc.
    pub fn price_pressure(&self) -> MarketSignal {
        let net = self.demand_signal.to_i8() - self.supply_signal.to_i8();
        MarketSignal::from_i8(net.clamp(-1, 1))
    }

    /// Simple price update based on pressure.
    pub fn update_price(&mut self) -> f64 {
        let pressure = self.price_pressure();
        let delta = pressure.to_i8() as f64 * 0.01;
        self.price = (self.price + delta * self.price).max(0.01);
        self.price
    }

    /// Supply-demand imbalance score (-2 to 2).
    pub fn imbalance(&self) -> i8 {
        self.demand_signal.to_i8() - self.supply_signal.to_i8()
    }
}

/// Portfolio optimizer with risk levels.
#[derive(Clone, Debug)]
pub struct PortfolioOptimizer {
    pub risk_level: RiskLevel,
}

impl PortfolioOptimizer {
    pub fn new(risk: RiskLevel) -> Self {
        PortfolioOptimizer { risk_level: risk }
    }

    /// Allocate weights across assets based on risk level and signals.
    pub fn allocate(&self, assets: &[Asset]) -> Vec<(String, f64)> {
        if assets.is_empty() {
            return vec![];
        }
        let risk_w = self.risk_level.weight();

        let mut scores: Vec<(String, f64)> = assets.iter().map(|a| {
            let signal_score = (a.signal.to_i8() as f64 + 1.0) / 2.0; // 0, 0.5, or 1.0
            let risk_adj = a.volatility * risk_w;
            let score = signal_score * (1.0 + a.expected_return) * (1.0 - risk_adj);
            (a.name.clone(), score.max(0.0))
        }).collect();

        let total: f64 = scores.iter().map(|(_, s)| *s).sum();
        if total == 0.0 {
            let equal = 1.0 / assets.len() as f64;
            return assets.iter().map(|a| (a.name.clone(), equal)).collect();
        }

        scores.iter_mut().for_each(|(_, s)| *s /= total);
        scores
    }

    /// Calculate expected portfolio return.
    pub fn expected_return(&self, assets: &[Asset]) -> f64 {
        let weights = self.allocate(assets);
        if weights.is_empty() { return 0.0; }
        let mut ret = 0.0;
        for (name, w) in &weights {
            if let Some(a) = assets.iter().find(|a| &a.name == name) {
                ret += w * a.expected_return;
            }
        }
        ret
    }
}

/// Risk assessment for a portfolio.
pub struct RiskAssessment {
    pub level: RiskLevel,
    pub max_volatility: f64,
}

impl RiskAssessment {
    pub fn new(level: RiskLevel, max_volatility: f64) -> Self {
        RiskAssessment { level, max_volatility }
    }

    /// Assess whether the portfolio is acceptable under this risk profile.
    pub fn assess(&self, assets: &[Asset]) -> RiskVerdict {
        let max_vol = assets.iter().map(|a| a.volatility).fold(0.0_f64, f64::max);
        let avg_return = if assets.is_empty() { 0.0 } else {
            assets.iter().map(|a| a.expected_return).sum::<f64>() / assets.len() as f64
        };

        match self.level {
            RiskLevel::Avoid => {
                if max_vol > self.max_volatility {
                    RiskVerdict::Reject("Volatility too high for risk-averse profile".to_string())
                } else if avg_return < 0.0 {
                    RiskVerdict::Reject("Negative expected return".to_string())
                } else {
                    RiskVerdict::Accept
                }
            }
            RiskLevel::Neutral => {
                if max_vol > self.max_volatility * 2.0 {
                    RiskVerdict::Reject("Volatility exceeds 2x threshold".to_string())
                } else if avg_return < -0.2 {
                    RiskVerdict::Caution("Low expected returns".to_string())
                } else {
                    RiskVerdict::Accept
                }
            }
            RiskLevel::Embrace => {
                if max_vol > self.max_volatility * 4.0 {
                    RiskVerdict::Caution("Extreme volatility".to_string())
                } else {
                    RiskVerdict::Accept
                }
            }
        }
    }
}

/// Risk assessment verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiskVerdict {
    Accept,
    Caution(String),
    Reject(String),
}

/// A ternary trading agent.
#[derive(Clone, Debug)]
pub struct TernaryAgent {
    pub id: usize,
    pub risk_tolerance: RiskLevel,
    pub signal: MarketSignal,
    pub cash: f64,
    pub holdings: f64,
}

impl TernaryAgent {
    pub fn new(id: usize, risk: RiskLevel, cash: f64) -> Self {
        TernaryAgent {
            id,
            risk_tolerance: risk,
            signal: MarketSignal::Neutral,
            cash,
            holdings: 0.0,
        }
    }

    /// Generate signal based on market data.
    pub fn generate_signal(&mut self, market_signal: MarketSignal) {
        // Risk-seeking agents amplify signals, risk-averse dampen
        let base = market_signal.to_i8();
        let adjusted = match self.risk_tolerance {
            RiskLevel::Avoid => {
                if base > 0 { 0 } else { base } // cautious: only act on bearish
            }
            RiskLevel::Neutral => base,
            RiskLevel::Embrace => {
                if base >= 0 { 1 } else { base } // aggressive: bias bullish
            }
        };
        self.signal = MarketSignal::from_i8(adjusted);
    }

    /// Execute a trade based on current signal and price.
    pub fn trade(&mut self, price: f64) -> TradeAction {
        match self.signal {
            MarketSignal::Bullish => {
                let amount = self.cash * self.risk_tolerance.weight();
                if amount > 0.01 {
                    let shares = amount / price;
                    self.holdings += shares;
                    self.cash -= amount;
                    TradeAction::Buy(shares)
                } else {
                    TradeAction::Hold
                }
            }
            MarketSignal::Bearish => {
                let sell_fraction = self.risk_tolerance.weight();
                let shares = self.holdings * sell_fraction;
                if shares > 0.01 {
                    self.holdings -= shares;
                    self.cash += shares * price;
                    TradeAction::Sell(shares)
                } else {
                    TradeAction::Hold
                }
            }
            MarketSignal::Neutral => TradeAction::Hold,
        }
    }

    /// Net worth at current price.
    pub fn net_worth(&self, price: f64) -> f64 {
        self.cash + self.holdings * price
    }
}

/// Trade action result.
#[derive(Clone, Debug, PartialEq)]
pub enum TradeAction {
    Buy(f64),
    Sell(f64),
    Hold,
}

/// Market simulation engine.
pub struct MarketSimulation {
    pub agents: Vec<TernaryAgent>,
    pub price: f64,
    pub history: Vec<f64>,
}

impl MarketSimulation {
    pub fn new(agents: Vec<TernaryAgent>, initial_price: f64) -> Self {
        MarketSimulation {
            agents,
            price: initial_price,
            history: vec![initial_price],
        }
    }

    /// Run one simulation step.
    pub fn step(&mut self, external_signal: MarketSignal) {
        let mut buys = 0.0_f64;
        let mut sells = 0.0_f64;

        for agent in &mut self.agents {
            agent.generate_signal(external_signal);
            let action = agent.trade(self.price);
            match action {
                TradeAction::Buy(amount) => buys += amount,
                TradeAction::Sell(amount) => sells += amount,
                TradeAction::Hold => {}
            }
        }

        // Price adjustment based on net buying pressure
        let net = buys - sells;
        let price_change = net * 0.1; // sensitivity factor
        self.price = (self.price + price_change).max(0.01);
        self.history.push(self.price);
    }

    /// Run multiple steps.
    pub fn run(&mut self, signals: &[MarketSignal]) {
        for &signal in signals {
            self.step(signal);
        }
    }

    /// Final price.
    pub fn final_price(&self) -> f64 {
        self.price
    }

    /// Price return over simulation.
    pub fn total_return(&self) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        (self.price - self.history[0]) / self.history[0]
    }

    /// Volatility (standard deviation of returns).
    pub fn volatility(&self) -> f64 {
        if self.history.len() < 2 { return 0.0; }
        let returns: Vec<f64> = self.history.windows(2)
            .map(|w| (w[1] - w[0]) / w[0])
            .collect();
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
        variance.sqrt()
    }
}

/// Trading strategy: momentum (follow the trend).
pub fn momentum_strategy(signals: &[MarketSignal]) -> Vec<TradeAction> {
    signals.iter().map(|&s| match s {
        MarketSignal::Bullish => TradeAction::Buy(1.0),
        MarketSignal::Bearish => TradeAction::Sell(1.0),
        MarketSignal::Neutral => TradeAction::Hold,
    }).collect()
}

/// Trading strategy: contrarian (opposite of signal).
pub fn contrarian_strategy(signals: &[MarketSignal]) -> Vec<TradeAction> {
    signals.iter().map(|&s| match s {
        MarketSignal::Bullish => TradeAction::Sell(1.0),
        MarketSignal::Bearish => TradeAction::Buy(1.0),
        MarketSignal::Neutral => TradeAction::Hold,
    }).collect()
}

/// Trading strategy: buy and hold.
pub fn buy_and_hold_strategy(len: usize) -> Vec<TradeAction> {
    let mut actions = vec![TradeAction::Buy(1.0)];
    for _ in 1..len {
        actions.push(TradeAction::Hold);
    }
    actions
}

/// Compute aggregate market signal from a history.
pub fn aggregate_history(signals: &[MarketSignal]) -> MarketSignal {
    let sum: i8 = signals.iter().map(|s| s.to_i8()).sum();
    MarketSignal::from_i8(sum.signum() as i8 * i8::min(sum.abs(), 1))
}

/// Sharpe-like ratio for a simulation.
pub fn sharpe_ratio(sim: &MarketSimulation, risk_free_rate: f64) -> f64 {
    let ret = sim.total_return();
    let vol = sim.volatility();
    if vol < 1e-10 { return 0.0; }
    (ret - risk_free_rate) / vol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_signal_conversion() {
        assert_eq!(MarketSignal::Bullish.to_i8(), 1);
        assert_eq!(MarketSignal::Neutral.to_i8(), 0);
        assert_eq!(MarketSignal::Bearish.to_i8(), -1);
        assert_eq!(MarketSignal::from_i8(1), MarketSignal::Bullish);
    }

    #[test]
    fn test_aggregate_signal_bullish() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bullish, 0.1, 0.05),
            Asset::new("B", MarketSignal::Bullish, 0.2, 0.08),
            Asset::new("C", MarketSignal::Neutral, 0.1, 0.02),
        ];
        let market = TernaryMarket::new(assets);
        assert_eq!(market.overall_signal, MarketSignal::Bullish);
    }

    #[test]
    fn test_aggregate_signal_bearish() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bearish, 0.1, -0.05),
            Asset::new("B", MarketSignal::Bearish, 0.2, -0.08),
        ];
        let market = TernaryMarket::new(assets);
        assert_eq!(market.overall_signal, MarketSignal::Bearish);
    }

    #[test]
    fn test_market_average_return() {
        let assets = vec![
            Asset::new("A", MarketSignal::Neutral, 0.1, 0.10),
            Asset::new("B", MarketSignal::Neutral, 0.2, 0.20),
        ];
        let market = TernaryMarket::new(assets);
        assert!((market.average_return() - 0.15).abs() < 1e-9);
    }

    #[test]
    fn test_supply_demand_pressure() {
        let sd = SupplyDemand::new(MarketSignal::Bearish, MarketSignal::Bullish, 100.0);
        // High demand, low supply → bullish
        assert_eq!(sd.price_pressure(), MarketSignal::Bullish);
    }

    #[test]
    fn test_supply_demand_price_update() {
        let mut sd = SupplyDemand::new(MarketSignal::Bearish, MarketSignal::Bullish, 100.0);
        let old_price = sd.price;
        let new_price = sd.update_price();
        assert!(new_price > old_price);
    }

    #[test]
    fn test_supply_demand_imbalance() {
        let sd = SupplyDemand::new(MarketSignal::Bearish, MarketSignal::Bullish, 100.0);
        assert_eq!(sd.imbalance(), 2); // 1 - (-1)
    }

    #[test]
    fn test_portfolio_allocate() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bullish, 0.1, 0.05),
            Asset::new("B", MarketSignal::Bearish, 0.5, -0.05),
        ];
        let opt = PortfolioOptimizer::new(RiskLevel::Avoid);
        let weights = opt.allocate(&assets);
        assert_eq!(weights.len(), 2);
        let total: f64 = weights.iter().map(|(_, w)| *w).sum();
        assert!((total - 1.0).abs() < 1e-9);
        // Risk-averse should favor low-volatility bullish asset
        assert!(weights[0].1 > weights[1].1);
    }

    #[test]
    fn test_portfolio_expected_return() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bullish, 0.1, 0.10),
            Asset::new("B", MarketSignal::Neutral, 0.1, 0.05),
        ];
        let opt = PortfolioOptimizer::new(RiskLevel::Neutral);
        let ret = opt.expected_return(&assets);
        assert!(ret > 0.0);
    }

    #[test]
    fn test_risk_assessment_accept() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bullish, 0.1, 0.05),
        ];
        let ra = RiskAssessment::new(RiskLevel::Avoid, 0.5);
        assert_eq!(ra.assess(&assets), RiskVerdict::Accept);
    }

    #[test]
    fn test_risk_assessment_reject() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bullish, 0.9, 0.05),
        ];
        let ra = RiskAssessment::new(RiskLevel::Avoid, 0.5);
        // Volatility 0.9 > max 0.5
        match ra.assess(&assets) {
            RiskVerdict::Reject(_) => {},
            _ => panic!("Expected Reject"),
        }
    }

    #[test]
    fn test_risk_assessment_negative_return() {
        let assets = vec![
            Asset::new("A", MarketSignal::Bearish, 0.1, -0.5),
        ];
        let ra = RiskAssessment::new(RiskLevel::Avoid, 0.5);
        match ra.assess(&assets) {
            RiskVerdict::Reject(_) => {},
            _ => panic!("Expected Reject for negative return"),
        }
    }

    #[test]
    fn test_agent_trade_bullish() {
        let mut agent = TernaryAgent::new(1, RiskLevel::Embrace, 1000.0);
        agent.generate_signal(MarketSignal::Bullish);
        let action = agent.trade(100.0);
        match action {
            TradeAction::Buy(_) => {},
            _ => panic!("Expected Buy"),
        }
        assert!(agent.cash < 1000.0);
        assert!(agent.holdings > 0.0);
    }

    #[test]
    fn test_agent_net_worth() {
        let agent = TernaryAgent::new(1, RiskLevel::Neutral, 500.0);
        // holdings = 0, so net worth = cash
        assert!((agent.net_worth(100.0) - 500.0).abs() < 1e-9);
    }

    #[test]
    fn test_simulation_run() {
        let agents = vec![
            TernaryAgent::new(1, RiskLevel::Embrace, 1000.0),
            TernaryAgent::new(2, RiskLevel::Avoid, 1000.0),
        ];
        let mut sim = MarketSimulation::new(agents, 100.0);
        sim.run(&[MarketSignal::Bullish, MarketSignal::Bullish, MarketSignal::Neutral]);
        assert!(sim.history.len() >= 4); // initial + 3 steps
        assert!(sim.final_price() > 0.0);
    }

    #[test]
    fn test_momentum_strategy() {
        let actions = momentum_strategy(&[
            MarketSignal::Bullish, MarketSignal::Bearish, MarketSignal::Neutral,
        ]);
        assert_eq!(actions[0], TradeAction::Buy(1.0));
        assert_eq!(actions[1], TradeAction::Sell(1.0));
        assert_eq!(actions[2], TradeAction::Hold);
    }

    #[test]
    fn test_contrarian_strategy() {
        let actions = contrarian_strategy(&[
            MarketSignal::Bullish, MarketSignal::Bearish,
        ]);
        assert_eq!(actions[0], TradeAction::Sell(1.0));
        assert_eq!(actions[1], TradeAction::Buy(1.0));
    }

    #[test]
    fn test_buy_and_hold() {
        let actions = buy_and_hold_strategy(5);
        assert_eq!(actions.len(), 5);
        assert_eq!(actions[0], TradeAction::Buy(1.0));
        for a in &actions[1..] {
            assert_eq!(*a, TradeAction::Hold);
        }
    }

    #[test]
    fn test_sharpe_ratio() {
        let agents = vec![TernaryAgent::new(1, RiskLevel::Neutral, 1000.0)];
        let mut sim = MarketSimulation::new(agents, 100.0);
        sim.run(&[MarketSignal::Bullish, MarketSignal::Bearish, MarketSignal::Bullish]);
        let sr = sharpe_ratio(&sim, 0.0);
        // Just verify it computes without panic
        assert!(sr.is_finite() || sr == 0.0);
    }

    #[test]
    fn test_risk_level_weights() {
        assert!(RiskLevel::Avoid.weight() < RiskLevel::Neutral.weight());
        assert!(RiskLevel::Neutral.weight() < RiskLevel::Embrace.weight());
    }

    #[test]
    fn test_aggregate_history() {
        let signals = vec![MarketSignal::Bullish, MarketSignal::Bullish, MarketSignal::Bearish];
        let agg = aggregate_history(&signals);
        assert_eq!(agg, MarketSignal::Bullish);
    }

    #[test]
    fn test_aggregate_history_neutral() {
        let signals = vec![MarketSignal::Bullish, MarketSignal::Bearish];
        let agg = aggregate_history(&signals);
        assert_eq!(agg, MarketSignal::Neutral);
    }

    #[test]
    fn test_agent_hold_on_neutral() {
        let mut agent = TernaryAgent::new(1, RiskLevel::Neutral, 1000.0);
        agent.generate_signal(MarketSignal::Neutral);
        let action = agent.trade(100.0);
        assert_eq!(action, TradeAction::Hold);
    }

    #[test]
    fn test_market_empty() {
        let market = TernaryMarket::new(vec![]);
        assert_eq!(market.overall_signal, MarketSignal::Neutral);
        assert_eq!(market.average_return(), 0.0);
    }
}
