🛡️ Aegis: The Social Equity Shield
===================================

**Protecting Global Property Equity via Pacifica Perpetual Hedging**

🌍 The Problem: A Global "Equity Wipeout" Risk
----------------------------------------------

While Aegis is being piloted in Leeds, UK, the problem is global. Whether it's **Purchase Lease Options (UK)**, **Rent-to-Own (USA)**, or **Off-plan Deposits (Global)**, millions of aspirational homeowners face the same "Leverage Trap":

-   **The Invisible Risk:** A buyer locks in a price with a 5-10% deposit. Because they do not yet hold the deed, a small market correction (e.g., 5%) wipes out **100% of their life savings.**

-   **The Gap:** Traditional insurance and banks do not protect against "Negative Equity" for non-deeded occupiers.

🚀 The Solution: Aegis
----------------------

Aegis is an automated "Headless" hedging engine built on the **Pacifica SDK**. It bridges physical real estate and DeFi liquidity to provide a digital safety net.

### Key Features:

-   **Autonomous Oracle Monitoring:** Watches property market data (Leeds Index, Case-Shiller, etc.) for volatility.

-   **Delta-Neutral Hedging:** Automatically opens a 5x Short on Pacifica (SOL-PERP) when a price drop is detected.

-   **Agent-Key Security:** Uses Pacifica's Ed25519 "Agent Wallet" architecture for secure, programmatic signing.

* * * * *

🛠️ Tech Stack & Requirements
-----------------------------

-   **Python 3.10+**

-   **Pacifica SDK**

-   **Active Internet Connection** (for Testnet API calls)

🏃 Installation & How to Run
----------------------------

### 1\. Clone & Setup

First, clone the repository and navigate into the project folder:
```bash
git clone [https://github.com/benpaymaster/aegis-finance.git](https://github.com/benpaymaster/aegis-finance.git)
cd aegis-finance
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

```

### 2\. Configuration

Open `pacifica_hedge.py` and ensure your **Agent Keys** are set:

Python

```
AGENT_SECRET = "YOUR_KEY"
MAIN_ACCOUNT = "YOUR_WALLET"

```

### 3\. Execution

**To run the main hedge engine:**

Bash

```
python pacifica_hedge.py

```

**To run the Market Crash Simulation (for Demo purposes):**

Bash

```
python simulate_crash.py

```

* * * * *

📊 The Math (Leeds Pilot Example)
---------------------------------

| **Market Scenario** | **Physical Equity Change** | **Pacifica Hedge Result** | **Net Position** |
| --- | --- | --- | --- |
| **Stable Market** | £0 | No Trade | **Protected** |
| **-5% Market Crash** | -£10,000 | +£10,000 (Short Profit) | **Hedged (Total £10k)** |

📖 Lexicon (Glossary)
---------------------

-   **PLO / Rent-to-Own:** Contracts where a tenant pays for the future right to buy a property.

-   **Option Fee:** The upfront capital (deposit) at risk of being wiped out by market dips.

-   **Delta-Neutral:** Offsetting physical asset loss with an equal gain in a DeFi position.

-   **Agent Key:** A restricted key for automated trading via Pacifica.

* * * * *

*Built for the Pacifica x DoraHacks 2026 Hackathon.*