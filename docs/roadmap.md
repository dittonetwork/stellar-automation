# Roadmap and budget

**Request: $120,000 USD equivalent in XLM over four months.** Weeks start on receipt of the approval advance. This is the delivery plan for the application, not an award or deployment announcement.

SCF pays $12,000 on approval, then $24,000 / $36,000 / $48,000 against the three milestones. The approval advance is included once in milestone 1's work budget.

| Milestone | Timing | Work budget | Completion evidence |
| --- | --- | ---: | --- |
| 1. Native testnet vault | Weeks 1–5 | $36,000 | Published contracts and tagged source; deposit, allocation and redemption demo; accounting and authorization tests |
| 2. Automated testnet platform | Weeks 6–10 | $36,000 | Operator failover, two-pool reallocation, reusable SDK and registry, Base funding demo, STRIDE model and monitoring plan |
| 3. Capped mainnet launch | Weeks 11–16 | $48,000 | Independently reviewed release, resolved critical/high findings, public deployment manifest and 14 days of monitored operation |

## Task budgets

| Milestone | Task | USD |
| --- | --- | ---: |
| 1 | Vault contracts and accounting | 15,000 |
| 1 | One Blend testnet pool and adapter | 13,000 |
| 1 | Wallet, deposit, position and redemption interface | 8,000 |
| 2 | Operator connector and two-pool allocation | 12,000 |
| 2 | Job registry and public SDK | 10,000 |
| 2 | Base-to-Stellar CCTP onboarding | 8,000 |
| 2 | STRIDE threat model, security checks and monitoring plan | 6,000 |
| 3 | Release hardening and external-review remediation | 20,000 |
| 3 | Mainnet deployment and application release | 18,000 |
| 3 | Monitoring and launch verification | 10,000 |
| **Total** | | **120,000** |

The estimate is 1,500 team-hours at a blended $80/hour. Marketing, external audit fees and past work are excluded.

## Review and release schedule

Arrange independent review and its separate funding at kickoff; confirm the slot by week 2. Freeze security-critical scope by week 8 and deliver the complete review candidate by week 10. Target review in weeks 10–12, reserve weeks 13–14 for remediation/retesting, and monitor the live capped release during weeks 15–16.

Mainnet deposits open only after review and resolution of all critical/high findings. If review is delayed, agree revised delivery dates with SCF; do not shorten review or the 14-day observation period.

## Launch acceptance

- Five successful deposit-to-redemption cycles from at least three disclosed test accounts, including one Base-funded deposit.
- At least two successful policy-valid actions from different executor accounts; confirmed cancellation blocks later execution.
- Fourteen consecutive monitored days after deposits open, with an incident runbook and published evidence.
- Contract IDs, code hashes, relevant accounts and transaction hashes published in a deployment manifest. Count executions by unique `(job ID, nonce)`; distinguish test activity from real external usage.

Pilot adoption target: by the end of the first 14 days after mainnet deposits open, reach 10 external funded wallets and 10,000 USDC in assets attributable to external depositors. These are targets for a small invited pilot, not committed demand or additional technical acceptance conditions. SCF must approve the final milestone terms. Count distinct non-team, non-test addresses with a positive vault-share balance at the day-14 ledger; calculate their assets as their share of vault net assets. Publish the ledger, contract IDs, calculation and excluded team/test accounts. Report actual results even if the targets are missed. Report CCTP-attributed inflows separately and do not count a wallet transfer again as a vault deposit.

## Scope boundaries

The platform uses venue adapters; Blend is the initial candidate for the one funded lending integration. Testnet uses two isolated pools to prove allocation logic; mainnet uses only reviewed, active USDC pools with adequate liquidity. A single qualifying pool is sufficient for a capped launch. SCF's current treatment of Blend in its Integration Track list requires clarification for this Open Track application; no approval is implied.

DeFindex and Templar are existing Stellar yield products, not interchangeable fallback adapters. Any venue replacement requires a revised custody/liquidity analysis, architecture and agreement with SCF. A DeFindex strategy using Blend would still have Blend exposure.

Additional Stellar lending venues and eligible RWA adapters are future work. Ondo USDY and Franklin Templeton BENJI are examples of Stellar RWA products to evaluate, not committed integrations. Access, valuation, liquidity and redemption rules must be reviewed separately.

## Pre-award work excluded

A minimal deposit/withdraw proof using native testnet XLM was completed before the award at Ditto's own expense. Its source, tests and transaction evidence are public. This proof is excluded from the grant budget; it does not implement USDC integration, yield shares, pool accounting, operators or CCTP. See [proof and scope](../prototype/README.md).
