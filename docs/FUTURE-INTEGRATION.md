# Future Integration: ternary-econ

## Current State
Provides ternary market analysis (bull/neutral/bear), supply/demand with ternary indicators, portfolio optimization, risk assessment, market simulation with ternary agents, and trading strategies.

## Integration Opportunities

### With ternary-room (Resource Allocation)
Rooms compete for compute resources (Codespace budget, GPU time, memory). This IS a market. `MarketSignal::Bull` = room needs more resources, `Bear` = room needs less, `Neutral` = room is balanced. `Portfolio` optimization distributes the fleet's compute budget across rooms for maximum total utility. `RiskLevel` assessment determines which rooms get guaranteed resources vs. spot instances.

### With ternary-econ → ternary-energy
Energy budgets ARE an economy. `TernaryMarket` where the commodity is compute energy. Supply = available GPU/CPU cycles, demand = room compute needs. Price discovery through market simulation determines which rooms get priority access to limited resources.

### With ternary-game-theory
Market simulation IS a game. Nash equilibrium of the resource allocation game determines the stable distribution. Cooperative game theory (Shapley values) determines each room's fair share of the fleet's resources.

## Potential in Mature Systems
In room-as-codespace, the fleet is an economy. Codespaces are assets with costs and returns. Portfolio optimization says: don't put all your compute in one room-type. Risk assessment says: high-variance rooms need resource reserves. Market simulation predicts resource demand and pre-provisions Codespaces.

## Cross-Pollination Ideas
- Trading strategies as room scheduling policies: momentum (keep investing in growing rooms), mean-reversion (pull back from rooms that grew too fast)
- Risk assessment as room health scoring —高风险 rooms need more monitoring
- Supply/demand matching for ensign allocation — which specialist goes to which room based on demand signals

## Dependencies for Next Steps
- ternary-room needs resource demand signaling
- Integration with ternary-energy for energy-denominated markets
- ternary-scheduling integration for market-driven resource scheduling
