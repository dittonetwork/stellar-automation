# Security design

**Status: initial design-stage analysis.** This is not an audit, a completed security assessment or proof that controls have been implemented. The full versioned STRIDE model, test results and monitoring configuration are milestone 2 deliverables.

Assets are user USDC, vault shares, pool positions, job ownership, execution nonces and signing authority. Trust boundaries separate wallets, the web application, RPC providers, operator processes, admin signers, Soroban contracts, Blend and Circle CCTP.

## Initial STRIDE analysis

| Threat class | Example | Required control and evidence |
| --- | --- | --- |
| Spoofing | Impersonate a job owner or executor | Onchain owner/executor authorization; negative signature and wrong-network tests |
| Tampering | Alter pool targets, deposit destination or accounting inputs | Fixed pool list, caps, validated CCTP configuration; parameter and accounting invariant tests |
| Repudiation | Dispute a job execution or policy change | Events and transaction receipts keyed by job, policy version and nonce; reproducible execution report |
| Information disclosure | Leak operator signing keys through logs | Local signing, restricted key storage, sanitized logs; deployment/key-handling review. Stellar activity is public |
| Denial of service | RPC outage, expired storage or unavailable pool liquidity | Retry/reconciliation, separately keyed executor instances, TTL/restoration checks; failover and illiquid redemption tests |
| Elevation of privilege | Executor changes policy or withdraws to itself | Separate user/admin/executor roles, registry-only rebalance, two-of-three admin; unauthorized-action tests |

Economic and integration risks need additional tests beyond this taxonomy: donation/share inflation, rounding, stale pool state, post-allocation rates, pool losses, USDC issuer controls, CCTP delays and duplicate transaction attempts.

## Controls to verify before deposits open

- All critical/high review findings resolved against the deployed code and configuration.
- Failed redemptions revert without burning shares; liquidity availability is not guaranteed.
- Job validation, action and nonce commit atomically. Cancellation blocks subsequent confirmed execution.
- Operators cannot redirect redemptions or change policy. Admin powers are disclosed with signer accounts.
- Pausing allocations/deposits leaves redemptions callable; fixed code prevents silent upgrades.
- Review covers storage lifetime, restoration, dependencies and cross-contract authorization.

## Monitoring plan

| Signal | Response to implement |
| --- | --- |
| Invalid/replayed execution or unexpected destination | Alert engineer; investigate receipt and enforce pause when integrity is uncertain |
| Repeated simulation/submission failures or missing executor heartbeat | Alert operator owner; reconcile pending transactions and use failover |
| Accounting mismatch or cap breach | Immediate engineer escalation; pause new deposits/allocations |
| Buffer or pool liquidity below policy limit | Alert and attempt only policy-valid liquidity recall; show redemption availability |
| CCTP transfer delayed or interrupted | Track original burn/message through completion; never initiate a replacement deposit automatically |
| Storage TTL below maintenance threshold | Extend or restore before execution; verify restored state |

The milestone 2 alert matrix will specify numeric thresholds, named owners and response times after baseline measurements. Demonstrations must include execution-failure and operator-outage alerts. Do not report these planned alerts as already operational.
