

## XCMP


### Payments

We allow for payments with native assets or with some configured rated foreign assets.
 

#### Acala

Can pay in KSM via next formula:

```python
number_of_instructions = 5
price_per_instructions = 100500
weight_per_message =  number_of_instructions * price_per_instructions 
native_to_ksm = 1/ 50
weight_per_second = 1_000_000_000_000
weight_of_remark = 125 * (weight_per_second / (1000 *1000)) 
native_unit  = 10**12
max_tx_per_second = weight_per_second / weight_of_remark

# this is weird
base_tx_in_native_unit = max_tx_per_second *  native_unit / 1000

total_weight_for_transaction  = weight_per_message /  weight_per_second

# SECONDS * (KSM / NATIVE) *(NATIVE/ SECONDS)    
# eventually this just 8x of message weigth, and transform decimals (so in default case these are 12 = 12)
required_ksm_amount = total_weight_for_transaction * native_to_ksm * base_tx_in_native_unit
```

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