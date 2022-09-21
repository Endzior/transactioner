# Transactioner

This repository contains a solution to an assignment given by one of the companies. This readme will contain some random bits and assumptions regarding the small app I wrote in this repo

## env_logger

Make sure RUST_LOG is not equal to debug or lower, that might pollute the output in the final binary. If it is not set - the default will be taken which is currently `warn`.

## Dispute can only occur on Deposit transactions

As stated in the provided coding document - a dispute should decrease available and increase held. It would make no sense if a Dispute could be filed on a Withdrawal - because then we would have magic money being detracted yet again.

## Dispute can lead to a a sitatuion of negative available funds

## Chargeback can lead to a situation of negative available funds

If such situation occurs for a single account:
- deposit
- withdrawal
- dispute the deposit
- chargeback the dispute

The available funds will be negative and the account locked

## Dispute can happen on same Deposit transaction multiple times

If the dispute will be followed by the same dispute, on same transaction then nothing happens - they just get overwritten by itself.

If the dispute will be resolved multiple times then nothing happens on the `Account` - they just get overwritten by itself.

If a chargeback occured - then nothing happens because we have already locked the `Account`.