# Acceptance test plan

**Status: planned tests, not executed results.** Implementation will add runnable tests, pinned toolchain/build instructions, tagged releases and reports linked to exact commits.

| Area | Required scenarios |
| --- | --- |
| Vault accounting | First deposit, donation/inflation resistance, share conversion and rounding, caps, empty vault, indexed pool valuation |
| Authorization | Unauthorized deposit/redemption, wrong executor, wrong vault/registry, admin/executor role separation |
| Redemption | Buffer-only exit, recall from pool, insufficient liquidity reverts without burned shares, pause leaves exit callable |
| Allocation | Two isolated testnet pools, rate change, post-allocation rate/cost estimate, exposure cap, cooldown, benefit threshold |
| Execution | Concurrent executors, duplicate nonce, stale policy, confirmed cancellation, retry after timeout and unknown submission outcome |
| Availability | One executor stopped, RPC failure, process restart, contract storage expiry/restoration |
| SDK reuse | A second team member deploys a second compatible vault, creates/runs/cancels its job without operator changes |
| CCTP | Base Sepolia to Stellar testnet, official forwarder destination, recipient readiness, decimal conversion, interrupted session and no duplicate deposit |
| Monitoring | Execution failure and executor outage produce the expected alert and runbook action |

For each integration scenario publish the network, code commit, relevant contract/account IDs, transaction hashes and expected versus actual result. Unit/integration test reports must distinguish local fixtures from live testnet transactions.

Mainnet acceptance is defined in the [roadmap](roadmap.md). Team-funded verification cycles are labeled tests; they are not counted as organic user traction. A successful test does not replace independent review.
