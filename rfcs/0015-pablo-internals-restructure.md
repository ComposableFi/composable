The [current implementation of `DualAssetConstantProduct`][dacp] is tightly
coupled with the pablo pallet, depending on pablo's `Config` trait directly for
all of it's functionality and directly modifying pablo's storage. This makes it
very difficult to test, since a full runtime is required to provide the required
mocks for it to work properly.

### Goals

1. Remove `DualAssetConstantProduct`'s direct dependency on pablo
2. Make all of the `DualAssetConstantProduct` functionality pure

### Implementation

Goal 1 is quite simple to implement, the required steps are outlined below:

1. Remove the `<T: Config>` bound from `DualAssetConstantProduct`, and move said
   bound to the associated functions on `DualAssetConstantProduct`.

2. For each associated function, review what functionality and types are
   actually required.

   As an example, for `add_liquidity`:

   - `T::Assets` as an implementor of various [`fungibles`][fungibles] traits,
     specifically using:

     - `Inspect::balance`
     - `Inspect::total_issuance`
     - `Mutate::mint_into`
     - `Transfer::transfer`

   - `Error::<T>` - uses the pallet's error type as it's error type, see **5.**
     for more on this.

   - `T::Convert` - converting between `T::Balance` and `u128` with
     [`Convert::convert`][convert] (which is just a convoluted version of
     `From`/`Into`).

   - `T::AssetId`, `T::AccountId`, `T::Balance` - generic types passed through
     from the pallet

     - `T::AssetId` and `T::AccountId` are opaque, and are just passed to the
       various functions that need them as-is (in this case, only `T::Assets`)

     - `T::Balance` is converted to `u128` for calculations, and then the
       results of those calculations are then converted back to `T::Balance` for
       use with `T::Assets`.

3. Change the signature of each associated function to require specifying all of
   the types and functionalities found in the previous step. Again using
   `add_liquidity` as an example, it's signature would now be this:

   ```rust
   fn add_liquidity<AccountId, AssetId, Balance, Assets, TConvert>(
       who: &AccountId,
       pool: BasicPoolInfo<AccountId, AssetId, ConstU32<2>>,
       pool_account: AccountId,
       assets: BiBoundedVec<AssetAmount<AssetId, Balance>, 1, 2>,
       min_mint_amount: Balance,
       keep_alive: bool,
   ) -> Result<(Balance, BTreeMap<AssetId, Balance>), DispatchError>
   where
       Assets: Inspect<AccountId, AssetId = AssetId, Balance = Balance>
           + Mutate<AccountId>
           + Transfer<AccountId>,
       TConvert: Convert<Balance, u128>,
       // these are required by BasicPoolInfo - these bounds will be discussed in
       // a separate task
       AccountId: Clone + PartialEq + Ord + Debug,
       AssetId: Ord + Clone + Debug,
   {
       // snip
   }
   ```

4. Every function should have it's own error type, defining all of the possible
   ways it could fail. For `add_liquidity`, it could look like this:

   ```rust
   enum AddLiquidityError {
       AssetNotFound,
       InitialDepositMustContainAllAssets,
       UnsupportedOperation,
       // ...etc

       // These are necessary since we depend on functionality that returns these
       // error types, such as the various fungibles::* traits:
       Arithmetic(ArithmeticError),
       Dispatch(DispatchError)),
   }
   ```

   Which would then replace `DispatchError` in the signature above, removing the
   dependency on pablo's `Error` type.

5. With the previous updates, the `DualAssetConstantProduct` struct is now
   unnecessary - the struct can be removed and it's associated functions can
   just be standalone functions.

---

Goal 2 is a bit more involved, as it requires removing all state accesses. This
is simple for pablo storages, as any reads can become parameters and writes can
be returned from the function; however non-pablo storage accesses are more
difficult as they are "indirect" - hidden inside a trait method called on a
generic type (for example, `T::Assets::mint_into(..)` incurs several storage
reads and writes, to different storages depending on the pallet used for
`T::Assets`).

Possible Solutions:

- **all** inputs and **all** outputs to the functions will be declared - this
  could however incur unnecessary storage reads if, for example, a value only
  needs to be read in certain branches.

[dacp]:
  https://github.com/ComposableFi/composable/blob/d90581d9349eb088ca930cf971dc05a21dca56b7/code/parachain/frame/pablo/src/dual_asset_constant_product.rs#L28
  "DualAssetConstantProduct"
[convert]:
  https://paritytech.github.io/substrate/master/sp_runtime/traits/trait.Convert.html
  "Convert trait"
[fungibles]:
  https://paritytech.github.io/substrate/master/frame_support/traits/tokens/fungibles/index.html
  "Fungibles traits"
