
## [Bribe DAO](https://www.bribe.xyz/)   

Bribe DAO is a pallet created in order for people to buy bribes and place bribes.  Allowing users to sell there votes and third parties to buy votes
for a certain amount.  


## Buying Votes    
Any user can buy votes by placing a bid for X amount of votes for Y amount of tokens.
BribeDAO will automatically find the votes for the amount and fullfill the request. 
Once the user has requested the buy the votes, the funds will be deposited and once the request is fullfilled and the votes has been picked out and bought, BribeDAO issues an offical vote with the help of pallet-democracy. 

Logic:
CreateBribeRequest > freeze/hold assets > match against votes > enact throw pallet-democracy > release funds to users > delete bribe  

A user place a bid for votes on a referendum 

## Selling Votes   
Any user can sell there votes on a certain proposal using the TakeBribeRequest function

Logic: 
TakeBribeRequest > register vote in FastMap vector   

Sell votes for a referendum

Left to do:
time limits


## Sorted Vectors   
BribeDAO sorts the votes and the amount in a custom vector as storage item, this vector is automatically sorted in order to give the users the most amount of votes for a certain amount. 
For example, if a users wants to buy 3 votes for 200 tokens, and there is two different option to choice from:
```rust
A = { votes: 3, tokens: 200, ref_index: 3 }
B = { votes: 3, tokens: 150, ref_index: 3 }
C = { votes: 3, tokens: 300, ref_index: 3 }

```

option B will automatically be selected, instead of just choicing the first hit.  


[Read more about the sorted vector implementation here](sortedvec.md)


## Future todo's
Time limit on bribe requests   
query the referendium to see if its active every time someone adds a vote or a bribe, throw democrazy pallet.   





