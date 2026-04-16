
# 🛡️ Aegis: The Social Equity Shield
**Combatting the Global Housing Crisis via Pacifica Perpetual Hedging**

## 📌 The Macro Crisis: "Generation Locked-Out"
As of April 2026, the global housing market has reached a point of systemic exclusion. Aegis was built in response to the "98% Trap"---a reality where the traditional "save and buy" model has fundamentally collapsed for the current generation.

### 📊 The Data of Exclusion (2025-2026):
* **The 98% Trap:** 98% of adults living with parents are priced out of their local markets ([Mortgage Soup](https://mortgagesoup.co.uk/millions-of-adults-trapped-at-home-as-98-priced-out-of-first-homes/)).
* **The 56-Year Wait:** A single earner in London now faces an impossible 56-year wait to save for a deposit ([Generation Rent](https://www.generationrent.org/2025/12/02/single-londoners-face-56-year-wait-to-buy-a-home/)).
* **Generation Locked-Out:** From Sydney to Tokyo, homeownership rates for under-35s have plummeted. In Tokyo, a condo costs **15x the average salary** ([Worldcrunch](https://worldcrunch.com/business-finance/young-people-homeowners/)).
* **The Valuation Gap:** In the UK, high house prices and high debt mean millions of aspiring owners rely on government funding or alternative paths like **Purchase Lease Options (PLOs)** ([The Times](https://www.thetimes.com/uk/politics/article/young-first-time-buyers-struggle-uk-mortgages-lv25vc3bj)).

## 🚀 The Solution: Aegis
Traditional finance and "light-touch" state involvement have failed. While cities like **Salford and Shanghai** are experimenting with state-led housing ([University of Manchester](https://www.manchester.ac.uk/about/news/from-salford-to-shanghai/)), Aegis provides a **DeFi-native bridge** for the individual.

Aegis uses the **Pacifica SDK** to protect the only path left for many: The "Price Lock" in alternative purchase agreements.

### Key Features:
* **Global Oracle Integration:** Monitors volatility in housing indices across major global hubs.
* **The Mortgage Bridge:** When property values dip, Aegis automatically opens a Short on Pacifica (SOL-PERP). The profit generated provides the **"Cash Bridge"** needed to complete a purchase when a bank's valuation falls below the contract's fixed price.
* **Agent-Key Security:** Leverages Pacifica's **Ed25519 Agent Keys** to allow the engine to manage protection autonomously without exposing the user's primary capital.

---

## 📊 The Math: Saving the Deal
| Market Scenario | Property Value | Bank Valuation | Pacifica Hedge Profit | Final Status |
| :--- | :--- | :--- | :--- | :--- |
| **Contract Start** | £200,000 | £200,000 | £0 | **Active** |
| **Market Dip (5%)** | £190,000 | £190,000 | **+£10,000** | **Deal Saved via Aegis** |

---

## 🛠️ Tech Stack
* **Engine:** Python 3.10+
* **DEX:** Pacifica Testnet (Perpetual Protocol)
* **Security:** Ed25519 Agent Signing
* **Dependencies:** `pacifica-sdk`, `cryptography`

## 🏃 Installation & Execution

### 1. Setup
```bash
git clone https://github.com/benpaymaster/aegis-finance.git
cd aegis-finance
python3 -m venv venv && source venv/bin/activate
pip install -r requirements.txt

```

### 2\. Configure & Run

Set your `AGENT_SECRET` in `pacifica_hedge.py` and execute:

```bash
# Start the Guardian Engine
python pacifica_hedge.py

# Test a global market crash simulation
python simulate_crash.py

```

* * * * *

*Built for the Pacifica x DoraHacks 2026 Hackathon. Protecting futures from Leeds to the World.*