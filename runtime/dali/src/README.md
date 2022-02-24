

## XCMP


### Payments

We allow for payments with native assets or with some configured rated foreign assets.
 

#### Acala


Acala follow next steps:

- use DEX to trade
  - DEX has has currency and there is enough to pay?
    - Yes. Pay
    - No. Go to next.
  - If there is hardcoded ratio?
    - Yes. Pay
    - No. Go to next.
  - If there is asset metadata with ability to have existential deposit payment in it?
    - Yes. Use ED as ratio for swap and pay if enough
    - No. Do not execute.

And has issues https://github.com/AcalaNetwork/Acala/issues/1921