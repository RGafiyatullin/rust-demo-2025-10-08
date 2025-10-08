
# Overview

The solution provides a single rust crate, that can either be run as a CLI, or used as a library.

The development process is demonstrated via the git and pull-request history.

# Self-Assessment

## Completeness

The solution supports all the requested transaction types:
- deposit
- withdraw
- dispute
- resolve
- chargeback.

## Correctness

Type-system is used to minimise the probability of an error:
- the amounts are kept in a fixed-number type (courtesy of the `fixnum` authors).
- amounts in transactions are ensured to be strictly positive.
- amounts are kept in separate non-negative fields for different types of transactions (i.e. deposited, withdrawn, disputed, etc), and the change is only accrued via addition (overflow checked).

Unit-tests have been added for the input-parsing and transaction-processing.

Integration tests have been added for the CLI.

## Safety and Robustness

Naturally, no unsafe code.

Separate error types used for the methods processing different types of transactions.

Alterations to the balance-fields are performed strictly after all the checks are done.

## Efficiency

The engine keeps two types of entieis:
- balances.
- transactions.

If env `ACCOUNT_PRUNING_ENABLED=1` is set, balances are pruned when possible (i.e. zero-balance, no funds held, not locked).

Transaction-IDs are recycled: if a transaction is not disputed (i.e. in the state Withdrawal or Deposited) — it may be pruned according to the LRU policy (Default cache size — 64M; Configurable via env `TX_LRU_SIZE`).

The engine does not require the whole input data set materialized in order to process it; it requires a single transaction at a time.

The balances of different accounts are independent, so if necessary, separate engines can be used to process distinct sets of accounts at the cost of allowing coinciding tx-ids in different shards.

## Maintainability

A clean git-history is preserved. The motivation of some seemingly weird choices can be traced back :)


# Assumptions

* it is assumed that 64M tx-id cache should be enough (estimated cache footprint — 2GiB).
* the way dispute behaviour is worded, it seems obvious that only `deposit`-transactions can be disputed.
* It is hoped for that `i128` will suffice to hold the amounts.
* transactions carrying amounts with precision exceeding 4-digits past decimal are rejected, rather than rounded to fit the chosen fixed-point number.
* the code is formatted using some `rustfmt.toml`. This approach is opinionated: I do not insist that this is the way to format the code; I just run rustfmt from time to time.
