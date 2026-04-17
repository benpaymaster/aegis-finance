"""
Aegis Macro-Basis Strategy Manager
Designed for Ranger Earn Build-A-Bear Track
"""

class AegisStrategy:
    def __init__(self, initial_capital=1000.0):
        self.base_asset = "USDC"
        self.capital = initial_capital
        self.tenor = "90-Day Rolling"
        self.leverage = 1.0
        self.is_active = True

    def get_alpha_signal(self, housing_index_price):
        """
        Uses the Aegis Housing Oracle to adjust basis trade leverage.
        Standard Basis Vaults are blind to macro credit cycles. We are not.
        """
        # Baseline is 100.0 (Stable)
        if housing_index_price >= 100.0:
            self.leverage = 3.0  # Max yield in stable/bull markets
        elif 95.0 <= housing_index_price < 100.0:
            self.leverage = 1.5  # De-risking during volatility
        else:
            self.leverage = 0.0  # 100% USDC Cash protection in a crash
        return self.leverage

    def calculate_projected_apy(self, funding_rate_hourly):
        """
        Requirement: >10% APY
        Targeting SOL/USDC funding capture
        """
        annualized_funding = funding_rate_hourly * 24 * 365
        net_apy = (annualized_funding * self.leverage) - 0.02 # 2% operation fee
        return max(net_apy, 0.0)

# Build-A-Bear Simulation
if __name__ == "__main__":
    aegis = AegisStrategy()
    print(f"Strategy Start: {aegis.base_asset} Base | {aegis.tenor} Lock")
    print(f"Leverage on Signal(85.0): {aegis.get_alpha_signal(85.0)}x")
    print(f"Projected Net APY: {aegis.calculate_projected_apy(0.00002) * 100:.2f}%")