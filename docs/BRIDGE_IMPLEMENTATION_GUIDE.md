# Bridge Implementation Guide

> **Status:** Work in progress. See [BRIDGE_INTEGRATION.md](./BRIDGE_INTEGRATION.md) for the
> currently supported integration patterns.

This guide will cover the step-by-step implementation of cross-chain bridge support within
StellarEscrow, including:

- Registering a bridge oracle via `set_bridge_oracle`
- Creating a cross-chain trade with `create_cross_chain_trade`
- Confirming bridge deposits via `confirm_bridge_deposit`
- Handling bridge trade expiry with `expire_bridge_trade`
- Error handling for bridge-specific `ContractError` variants (80–94)

For now, refer to:
- `contract/src/bridge.rs` — on-chain bridge logic
- `indexer/src/bridge_service/` — off-chain coordination layer
- `api/src/bridge.ts` — TypeScript bridge API client
