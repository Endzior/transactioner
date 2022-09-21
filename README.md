# Transactioner

This repository contains an assignment solution by one of the companies. This readme will contain some random bits and assumptions regarding the small app I will write here.


## Dispute can only occur on Deposit transactions

As stated in the provided coding document - a dispute should decrease available and increase held. It would make no sense if a Dispute could be filed on a Withdrawal - because then we would have magic money being detracted yet again.