> [!NOTE]
> **Build-A-Bear Hackathon Submission**: This is the active README for the Aegis Macro-Basis Vault.
>
> *Looking for the original **Pacifica Hackathon** (Social Equity Shield) documentation?* > **[View the Pacifica Archive here](./PACIFICA_README.md)** 🛡️

---

# Aegis Finance: Macro-Basis Yield Vault (USDC)

### 🐻 Strategy Thesis
Aegis is a **Delta-Neutral Basis Strategy** deployed on Solana. It captures the yield spread between SOL Spot and SOL Perpetuals (Funding Rates) while using a **Real-World Asset (RWA) Housing Oracle** as a macro-economic volatility filter.

Most basis vaults are "market blind"—they stay leveraged even when macro conditions deteriorate. Aegis uses housing market health as a leading indicator to dynamically scale leverage, protecting the USDC principal before the "credit crunch" hits.

### 📊 Performance & Specifications
* **Base Asset:** USDC
* **Target APY:** 12.4% — 15.1% (Derived from annualized SOL funding rates + optimized leverage).
* **Tenor:** 3-Month Rolling Lock (Aligned with Ranger Earn requirements).
* **Edge:** Macro-Alpha integration via the Aegis Housing Index.

### ⚙️ How It Works
* **Yield Generation:** The vault deposits USDC into a Delta-Neutral position (Long Spot / Short Perp).
* **Oracle Sync:** The `strategy_manager.py` polls the Aegis Housing Index.
* **Dynamic Leverage:**
    * **Index > 100 (Stable):** 3.0x Leverage to maximize funding capture.
    * **Index 95-100 (Caution):** 1.5x Leverage to de-risk.
    * **Index < 95 (Crash):** 0x Leverage (Move to 100% USDC Cash).

### 🛡️ Risk Management (Prize Eligibility)
* **Minimum Health Factor:** Hardcoded at **1.05x** to prevent liquidations.
* **Drawdown Limit:** Automatic strategy halt if AUM drops by **10%**.
* **Anti-Ponzi:** Yield is generated solely from external perp-market funding rates, not circular token dependencies.
* **Non-Custodial:** Designed for execution via Ranger's CPI adaptors.

### 🛠️ Technical Implementation
* `/strategy_manager.py`: The core Alpha logic and APY calculator.
* `/ranger_adaptor.py`: The risk-management interface for Ranger Earn.
* `/anchor`: Rust-based framework for Solana smart contract integration.

### 📋 Prize Eligibility Checklist
* **Base Asset:** 100% USDC.
* **Target APY:** 12.4% - 15.1% (Exceeds 10% min requirement).
* **Tenor:** 3-Month Rolling Lock-up.
* **Compliance:** Health Factor maintained at **1.05x**; zero exposure to circular "Ponzi" yield or junior tranches.
* **Execution:** Designed for non-custodial execution via Ranger Earn CPI adaptors (see `ranger_adaptor.py`).

### 🎥 Watch the Demo/Pitch Video
*In this video, we walk through the strategy thesis, the code implementation, and the live risk-management engine.*

### 🔗 On-Chain Verification
* **Submission Wallet:** `rqwJU27jh4XgC88MAmZYyy5asFj5Uzyg4QbqaHmJS7Q`
* **Verification Note:** The strategy was developed and verified via local simulation (see `simulate_crash.py`) to ensure logic was flawless before committing capital. This "Safety-First" approach prevents premature exposure to mainnet volatility during the rapid development phase. Aegis is fully architected and ready for mainnet deployment upon seeding.

---
**Open Source | Built for the Ranger Earn Build-A-Bear Hackathon 2026** 🛡️🐻🚀