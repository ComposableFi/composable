## ICS_20 - Fungible Token Transfer

Ics20 is an ibc application protocol that facilitates cross chain fungible token transfer.  
Ics20 requires that the host chain has a means to dynamically issue new tokens.  
Ics20 tokens are created on demand based on channel and port combinations, so the host chains must provide a means to  
mint tokens with arbitrary denominations.

### Ics20 Denoms

Denoms are a way to represent a unique cross chain token, to allow tracking of a token's flow, across multiple chains.   
denoms are created by appending the destination channel and port to the base token denomination. Say a token with a base denomination of `Mars`  
is transferred from chain A on channelA, port transfer to channelB on chain B, port transfer, the token denomination  
on receipt at chain b becomes `channelB/transfer/Mars`.  

### Packet Data

Ics20 has a standard format for packet data, ics20 data must be serialized to json before sending.  
The standard format for a sample json serialized ics20 data is as shown below

```json
    {
        "denom": "channelB/transfer/Mars",
        "amount": "22000",
        "sender": "EyhghTgxjajIkjht",
        "receiver": "EyhghTgxjajIkjht"
    }
```

### Acknowledgement

Ics20 requires an acknowledgement packet to be sent and defines an acknowledgement success type.  
A successful acknowledgement is defined by the string `AQ==`, any other acknowledgement value is interpreted as an error.

### ICS20 Context

Ics20 context defines the methods required for the ics20 module callbacks to execute correctly.  
Correct execution of the Ics20 protocol hinges on the correct implementation of these methods.  

**BankKeeper**
- **send_coins** - The implementation of this method should send tokens from one account to another.
- **mint_coins** - This method should be implemented to increase the total supply of new token by the specified amount and send that amount to an account.
- **burn_coins** - This method should reduce the token balance of a user and decrease the total supply of such token by the specified amount.  

**Ics20Reader**
- **get_port** - Should return the ics20 port.
- **get_channel_escrow_address** - Should derive an account Id from the channel and port 
- **is_send_enabled** - Should return a boolean that indicates if ics20 send action is allowed.
- **is_receive_enabled** Should return a boolean that indicates if ics20 token receipt is allowed.

### ICS20 Callbacks

