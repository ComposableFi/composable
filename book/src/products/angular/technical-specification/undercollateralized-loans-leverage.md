# Undercollateralized Loans/Leverage

In certain circumstances it is actually possible to provide a user with an undercollateralized loan. The funds transmitted to the user need to be put into custody, allowing the user to only interact with a limited set of protocols. Some examples of whitelisted operations:


*   Providing Liquidity, if the LP tokens are priceable.
*   Swapping between stablecoins.
*   Purchasing bonds if the offered asset is whitelisted, not vested and priceable.


An undercollateralized loan will allow a user to lock 10 ETH, obtain 100 ETH to then use in any of these use cases.


#### Risks to the protocol

This is a difficult to implement feature, as we need to make sure that the set of whitelisted operations does not allow the user to funnel away funds from the proxy. It is also quite vulnerable to black swan events.


#### Implementation

There would be two ways to implement this:


1.  A pallet which has a single whitelist in the configuration, of type:

```plain
pub trait Whitelist {
   fn all_is_valid(calls: impl Iterator<T::Call>) -> bool {
      calls.iter.any(!Self::is_valid)
   }
   fn is_valid(call: T::Call) -> bool;
}
```

The pallet would maintain sub\_accounts for users, which are allowed to proxy calls to the sub\_accounts, which are checked by the whitelist.


2\. Storing a Whitelist outside of the configuration, so that governance would be able to vote on adding/removing calls. It would resemble the functionality of a Regex.


Users can then proxy calls to their sub\_account:

```plain
#[transactional]
fn proxy(origin, sub_account, calls: Vec<T::Call>) -> DispatchResult {
    let user = ensure_signed(origin)?;
    assert(T::all_is_valid(calls))?;
    T::Executor::execute(user, sub_account, calls);
}
```


For each `sub_account`, we need to keep track of the total liquidatable value. The `Executor` can be used to inspect the calls and perform bookkeeping based on the call. It does not need to store the amount of tokens that the `sub_account` has, but it should store each AssetId.


When the total value of the `sub_account` drops below a certain value, a liquidation will be triggered, selling all assets stored by the account.


Some example operations which could be whitelisted/checked


1.  Transfer between sub\_accounts of same account
2.  Provide liquidity in exchange for LP/DEX tokens
3.  Exchange on DEX for whitelisted assets
4.  Exchange on DEX if final operations result in correct health factor:
    1.  Exchange PICA for SHIT (SHIT is not whitelisted)
    2.  Exchange SHIT for ACA (ACA is not whitelisted)
    3.  Exchange ACA for PICA (now we are in healthy state again).


Alternatively, we can abstractly model it like so:


A sub\_account is described by its state S. Each call transforms the state of S to S1, S2 etc. S\_final is the state obtained after executing all calls. The following guarantees must hold:


* S\_final must have a health factor > limit
* Intermediate states may have health factor < limit, and do not need to be checked.
* Health factor is the ratio between the borrow and total value of the sub\_account.
