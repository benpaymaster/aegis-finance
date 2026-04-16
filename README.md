# 🛡️ Aegis: The Social Equity Shield
**Protecting UK Tenant Deposits via Pacifica Perpetual Hedging**

## 📌 The Problem: The "Lease Option" Trap
In the UK, thousands of residents use **Purchase Lease Options (PLOs)** to get on the housing ladder. These tenants pay a significant upfront "option fee" (e.g., £10,000) for the right to buy a home at a fixed price in the future.

**The Invisible Risk:**
Lease options are effectively **20x leveraged positions** on physical property. 
* If a house in Leeds worth £200,000 drops just **5%** in value, the property is now worth £190,000. 
* The tenant’s entire £10,000 deposit is **wiped out** in negative equity. 
* High-street banks offer no protection for this specific niche.

## 🚀 The Solution: Aegis
Aegis is an automated "Headless" hedging engine built on the **Pacifica SDK**. It monitors real-world property indices and uses DeFi liquidity to provide a digital safety net for physical home-buyers.

### Key Features:
* **Autonomous Oracle Monitoring:** Watches local property market data for volatility thresholds.
* **Delta-Neutral Hedging:** Automatically opens a 5x Short on Pacifica (SOL-PERP) when a 2% price drop is detected.
* **Agent-Key Security:** Uses Pacifica's Ed25519 "Agent Wallet" architecture to sign programmatic trades without exposing main account funds.
* **Social Impact:** Offsets physical equity loss with DeFi trading profits, keeping the tenant's deposit "whole."

## 🛠️ Tech Stack
* **Engine:** Python 3.10+
* **DEX:** Pacifica Testnet (Perpetual Protocol)
* **Auth:** Ed25519 Agent Signing
* **Environment:** Virtualized Python Environment (venv)

## 🏃 Quick Start
1. **Activate Environment:** `source venv/bin/activate`
2. **Configure Keys:** Update `pacifica_hedge.py` with your Agent Secret.
3. **Execute Shield:** `python pacifica_hedge.py`

## 📊 The Math
| Market Scenario | Physical Equity Change | Pacifica Hedge Result | Net Position |
| :--- | :--- | :--- | :--- |
| **Stable Market** | £0 | No Trade | **Protected** |
| **-2.5% Leeds Dip** | -£5,000 | +£5,000 (Short Profit) | **Hedged (Total £10k)** |

## 📖 Lexicon (Glossary)
* **PLO (Purchase Lease Option):** A legal contract where a tenant pays for the right to buy a property at a set price in the future.
* **Option Fee:** The upfront capital paid by the tenant. In Aegis, this is the "at-risk" asset we are protecting.
* **Delta-Neutral:** A strategy where the loss in one asset (Property) is offset by an equal gain in another (Pacifica Short).
* **Agent Key:** A restricted cryptographic key that allows Aegis to trade autonomously without needing the user's main private key for every transaction.
* **Oracle:** The data bridge that brings real-world Leeds house price indices into the Aegis execution engine.

---
*Built for the Pacifica x DoraHacks 2026 Hackathon.*