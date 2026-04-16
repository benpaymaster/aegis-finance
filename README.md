🛡️ Aegis: The Social Equity Shield
===================================

**Combatting the Global Housing Crisis via Pacifica Perpetual Hedging**

📌 The Macro Crisis: "Generation Locked-Out"
--------------------------------------------

As of April 2026, the global housing market has reached a point of systemic exclusion. Aegis was built in response to the "98% Trap"---a reality where the traditional "save and buy" model has fundamentally collapsed for the current generation.

### 📊 The Data of Exclusion (2025-2026):

-   **The 98% Trap:** 98% of adults living with parents are priced out of their local markets ([Mortgage Soup](https://mortgagesoup.co.uk/millions-of-adults-trapped-at-home-as-98-priced-out-of-first-homes/)).

-   **The 56-Year Wait:** A single earner in London now faces an impossible 56-year wait to save for a deposit ([Generation Rent](https://www.generationrent.org/2025/12/02/single-londoners-face-56-year-wait-to-buy-a-home/)).

-   **Generation Locked-Out:** From Sydney to Tokyo, homeownership rates for under-35s have plummeted. In Tokyo, a condo costs **15x the average salary** ([Worldcrunch](https://worldcrunch.com/business-finance/young-people-homeowners/)).

-   **The Valuation Gap:** In the UK, millions of aspiring owners rely on alternative paths like **Purchase Lease Options (PLOs)** ([The Times](https://www.thetimes.com/uk/politics/article/young-first-time-buyers-struggle-uk-mortgages-lv25vc3bj)).

🚀 The Solution: Aegis
----------------------

Aegis provides a **DeFi-native bridge** for the individual. By using the **Pacifica SDK**, we allow tenant-buyers to protect their "Price Lock" equity from market volatility.

### Key Features:

-   **Global Oracle Integration:** Monitors housing indices across major global hubs.

-   **The Mortgage Bridge:** Automatically opens a Short on Pacifica (SOL-PERP) to generate the "Cash Bridge" needed when bank valuations fall.

-   **Agent-Key Security:** Uses **Ed25519 Agent Keys** to manage trades autonomously without risking the user's primary wallet.

* * * * *

📊 The Math: Saving the Deal
----------------------------

| **Market Scenario** | **Property Value** | **Bank Valuation** | **Pacifica Hedge Profit** | **Final Status** |
| --- | --- | --- | --- | --- |
| **Contract Start** | £200,000 | £200,000 | £0 | **Active** |
| **Market Dip (5%)** | £190,000 | £190,000 | **+£10,000** | **Deal Saved via Aegis** |

* * * * *

🛠️ Tech Stack
--------------

-   **Engine:** Python 3.10+

-   **DEX:** Pacifica Testnet

-   **Security:** Ed25519 Agent Signing

-   **Dependencies:** `pacifica-sdk`, `python-dotenv`, `cryptography`

🏃 Installation & Execution (Guide for Judges)
----------------------------------------------

### 1\. Setup Environment

Bash

```
git clone https://github.com/benpaymaster/aegis-finance.git
cd aegis-finance
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

```

### 2\. Configure Credentials

Aegis uses environment variables for security. Do not hardcode keys into the scripts.

1.  Create a `.env` file from the template:

    Bash

    ```
    cp .env.example .env

    ```

2.  Open `.env` and enter your Pacifica credentials:

    Plaintext

    ```
    AGENT_SECRET=your_agent_private_key
    MAIN_ACCOUNT=your_main_wallet_address

    ```

### 3\. Run the Engine

**To start the live Guardian Engine:**

Bash

```
python pacifica_hedge.py

```

**To run the Market Crash Simulation (Demo Mode):**

Bash

```
python simulate_crash.py

```

* * * * *

*Built for the Pacifica x DoraHacks 2026 Hackathon. Protecting futures from Leeds to the World.*