# Roadmap and budget

SCF Open Track. $96,000 USD equivalent in XLM, 1,200 team-hours at a blended $80/hour, 16 weeks from receipt of Tranche #0.

| Payment | Amount | Trigger |
| --- | ---: | --- |
| Tranche #0 | $9,600 | Approval advance |
| Tranche #1 | $19,200 | MVP acceptance |
| Tranche #2 | $28,800 | Integration acceptance |
| Tranche #3 | $38,400 | Mainnet acceptance |

The first work budget includes the advance once. Total work budgets: $28,800 + $28,800 + $38,400. External audit fees, marketing and past work are outside this request.

## Milestone 1

Milestone: a working Stellar USDC vault on testnet. Weeks 1-5 from receipt of Tranche #0. Work budget: $28,800 for 360 hours at $80/hour, funded by the $9,600 approval advance and $19,200 milestone payment.

1. Soroban vault contracts: 140 hours, $11,200. Implement USDC deposits, proportional shares, total-asset accounting, redemptions, owner authorization and deposit caps. Evidence: deployed testnet contracts and passing accounting, rounding, donation/first-deposit, unauthorized-call and failed-redemption tests.

2. Blend adapter: 100 hours, $8,000. Implement supply, withdrawal, position valuation and available-liquidity reads for a USDC pool. Evidence: linked supply and withdrawal transactions, correct vault valuation before and after interest accrual, and an illiquid-exit test preserving user shares.

3. Deposit application: 80 hours, $6,400. Deliver Freighter/xBull connection, deposit, share balance, position value and redemption screens. Evidence: a testnet URL and a recorded deposit-to-redemption flow.

4. Integrated MVP verification: 40 hours, $3,200. Verify the complete wallet, vault and adapter flow with reproducible tests, contract IDs and a tagged code version. This covers cross-component behavior, separately from the unit tests above.

The existing self-funded native-XLM prototype is excluded from this budget. This milestone builds the USDC yield vault and application beyond that proof. Operators and cross-chain deposits follow in milestone 2.

## Milestone 2

Milestone: automated allocation across two strategy types, with SDK and inbound funding integrations ready for review. Weeks 6-10. Budget and acceptance payment: $28,800 for 360 hours at $80/hour.

1. Operators, registry and SDK: 140 hours, $11,200. Add job creation, cancellation, status, policy validation and allocation execution. Evidence: a controlled rate-change scenario triggers reallocation; cap, cooldown and nonce tests pass; execution continues after one of two separately keyed operators stops. A second compatible testnet vault runs and cancels jobs through the same SDK without changing operator software.

2. Sentora USDC / Upshift adapter: 80 hours, $6,400. Add deposit, receipt-share valuation and redemption integration, with available-liquidity checks and a 20% limit on new or increased exposure. Evidence: tests against a testnet interface fixture covering share decimals, NAV changes, partial liquidity and failed exits. Record read-only checks against the live Stellar contract. Use a local Soroban fixture for repeatable tests and verify production interaction in milestone 3.

3. CCTP onboarding: 40 hours, $3,200. Integrate Base and Ethereum USDC transfer flows and Stellar CctpForwarder settlement, followed by a separate authorized vault deposit. Evidence: testnet burn/mint/deposit transactions for both source configurations; destination, trustline, decimal and interrupted-session recovery tests.

4. NEAR Intents onboarding: 40 hours, $3,200. Integrate 1Click quotes, destination/refund validation, status tracking and the Solana-USDC-to-Stellar-USDC flow. Evidence: live dry quotes plus deterministic settlement, expiry and refund fixtures. Verify live mainnet settlement in milestone 3.

5. STRIDE threat model and monitoring: 60 hours, $4,800. Document threats and controls for authorization, replay, valuation, managed-strategy liquidity, routing, RPC failure and storage lifetime. Evidence: a versioned model, security-test results, remediation log and demonstrated execution-failure/operator-outage alerts.

Freeze security-critical scope by week 8 and provide the complete review package by week 10. Reuse Ditto's existing operator scheduling, transaction tracking and application components; the funded work is their Stellar integration and verification.

## Milestone 3

Milestone: capped mainnet launch of Earn, operators and both yield adapters, followed by 14 days of verified operation. Weeks 11-16. Budget and acceptance payment: $38,400 for 480 hours at $80/hour.

1. Security hardening and review remediation: 180 hours, $14,400. Resolve findings, test storage restoration, access controls, share accounting and failed strategy exits. Evidence: external review report and remediation results for the deployed code and configuration. Mainnet deposits open only after external review and resolution of all critical and high findings. Audit fees are outside this grant request.

2. Mainnet deployment: 100 hours, $8,000. Deploy the vault, job registry, Blend and Sentora/Upshift adapters and two operator instances. Configure an initial 100,000 USDC vault deposit cap, a 10% idle-USDC target and a 20% limit on new or increased Sentora exposure. Publish contract IDs, code hashes, permitted strategies, admin/executor accounts and permissions.

3. Production integration verification: 100 hours, $8,000. Verify native deposits, Base and Ethereum CCTP deposits, and a Solana USDC deposit through NEAR Intents. Verify strategy deposits and redemptions through both adapters, operator allocation and cancellation. Evidence: linked source/settlement/vault transactions, measured settlement times, liquidity checks and a report separating fixture tests from live execution.

4. Launch observation and developer handoff: 100 hours, $8,000. Complete 14 consecutive days of monitored operation, publish the incident runbook, SDK quickstart and an embeddable deposit-component example. Evidence: five deposit-to-redemption cycles across three disclosed test accounts, at least two policy-valid operator actions from different executor accounts, and cancellation blocking subsequent execution. Team tests are reported separately from adoption.

Pilot growth targets for the first 14 days: 10 external funded wallets and 10,000 USDC in assets attributable to external depositors. Report actual performance against these targets separately from technical milestone acceptance. Measure daily balances and day-14 results using the published vault, registry, strategy and executor addresses, excluding disclosed team/test accounts.

Schedule: reserve independent review and its separate funding in weeks 1-2; review in weeks 10-12; fixes and retesting in weeks 13-14; open capped mainnet deposits at the start of week 15 and complete observation by the end of week 16. Public deposits open only after the review gate is passed.

## Onchain measurement

We will publish the Earn vault, job registry, strategy adapters, underlying strategy contracts, admin accounts and executor accounts in a deployment manifest.

Technical acceptance: five complete deposit-to-redemption cycles across at least three disclosed test accounts; successful native funding and Base/Ethereum CCTP and Solana-USDC NEAR Intents funding; deposit and redemption evidence for both strategy adapters; at least two policy-valid operator actions from different executor accounts; and cancellation blocking later execution. Complete 14 consecutive days of monitored mainnet operation after deposits open. Deduplicate operator actions by (job ID, nonce) and vault flows by transaction hash.

Pilot growth targets: 10 external funded wallets with positive vault-share balances and 10,000 USDC in assets attributable to external depositors at the day-14 ledger. Publish the ledger, daily balances, share-based asset calculation and excluded team/test accounts. A wallet address is not claimed to be a unique person. Report net deposits, cross-chain-attributed vault deposits and realized net yield separately, without counting a wallet transfer as a vault deposit. Report actual growth against these targets separately from the technical acceptance criteria.

[Task-by-task hours and budget](budget-hours.csv).
