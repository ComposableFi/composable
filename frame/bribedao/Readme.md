
## Bribe DAO

Bribe DAO is a pallet created in order for people to buy bribes and place bribes.  Allowing users to sell there votes and third parties to buy votes
for a certain amount.  


## Buying Votes    
Any user can buy votes by placing a bid for X amount of votes for Y amount of tokens.
BribeDAO will automatically find the votes for the amount and fullfill the request. 
Once the user has bought the votes, BribeDAO issues an offical vote with the help of pallet-democracy. 



## Selling Votes   
Any user can sell there votes on a certain proposal using the TakeBribeRequest function



## Sorted Vectors   
BribeDAO sorts the votes and the amount in a custom vector as storage item, this vector is automatically sorted in order to give the users the most amount of votes for a certain amount. 
For example, if a users wants to buy 3 votes for 200 tokens, and there is two different option to choice from:
A = { votes: 3, tokens: 200 }
B = { votes: 3, tokens: 150 }
C = { votes: 3, tokens: 300 }

option B will automatically be selected, instead of just choicing the first hit.  


[Read more about the sorted vector implementation here](sortedvec.md)

