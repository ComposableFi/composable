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
       // these are required by BasicPoolInfo - see note(1) for more information.
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

   > In the case of `add_liquidity` (and similarly, `remove_liquidity`), this
   > would also enable much easier error reporting for the `simulate_*` rpcs, as
   > the error types would already be defined.

5. With the previous updates, the `DualAssetConstantProduct` struct is now
   unnecessary - the struct can be removed and it's associated functions can be
   converted to standalone functions.

---

Goal 2 is a bit more involved, as it requires removing all state accesses. This
is simple for pablo storages, as any reads can become parameters and writes can
be returned from the function; however non-pablo storage accesses are more
difficult as they are "indirect" - hidden inside a trait method called on a
generic type (for example, `T::Assets::mint_into(..)` incurs several storage
reads and writes, to different storages depending on the pallet used for
`T::Assets`).

Possible Solutions:

- **All** inputs and **all** outputs to the functions will be declared. See
  [this file][explicit-example] for an example.

  Pros:

  - Full separation from the runtime - a
    [`TestExternalities`][test-externalities] is no longer required to test the
    functionality.
  - Inputs to and requirements of the functions are explicitly stated, resulting
    in them being easier to reason about.
  - Function is now entirely pure - no state modifications are done from within
    the function.

  Cons:

  - This could incur unnecessary storage reads if, for example, a value only
    needs to be read in certain branches.

- pass "hooks" to the functions, that would then be called whenever that value
  is needed. See [this file][hooks-example] for an example.

  Pros:

  - Full separation from the runtime - a
    [`TestExternalities`][test-externalities] is no longer required to test the
    functionality.
  - Inputs to and requirements of the functions are explicitly stated, resulting
    in them being easier to reason about.

  Cons:

  - Somewhat verbose, both in implementation and testing. The implementation
    requires writing out function signatures for every external input and
    testing requires creating something that can implement said interface (as
    seen in the linked example). This is also somewhat "not DRY" as the input
    definitions are essentially just redefining the trait methods they're
    replacing.
  - This isn't technically pure, as there are still state modifications done,
    just now hidden behind a function call instead of an associated method on a
    trait.

---

### Notes

1. This is a common pattern used throughout our codebase:

   ```rust
   #[derive(
       Encode,
       Decode,
       MaxEncodedLen,
       TypeInfo,
       CloneNoBound,
       DefaultNoBound,
       PartialEqNoBound,
       EqNoBound,
       RuntimeDebugNoBound,
   )]
   #[scale_info(skip_type_params(Bound))]
   pub struct Struct<
       T: Ord + Clone + Debug,
       U: Clone + PartialEq + Debug,
       V: Clone + PartialEq + Debug,
       Bound: Get<u32>,
   > {
       pub some_bounded_field: BoundedBTreeMap<T, U, MaxAssets>,
       pub generic_field: V,
   }
   ```

   Rust `derive`s eagerly add bounds to every generic parameter, which doesn't
   work for parameters that are "only type-level" such as `Bounds` seen above.
   To work around this, the [`*NoBound` derive macros][no-bound-derives] derive
   macros from parity require that the necessary bounds be placed on the types
   themselves, as the bounds from the definition are copied directly to the
   implementation.

   This is a bit of an abuse of these macros, as they were mostly intended to be
   used with structures generic over the runtime entirely (which this rfc is
   attempting to remove), and can result in some unintended consequences - in
   the above example, `Struct` will only implement `Clone` if `T`, `U`, and `V`
   all implement `Debug` (among other things).

   Taken directly from the docs for [PartialEqNoBound][partial-eq-no-bound]:

   > This is useful for type generic over runtime:
   >
   > ```rust
   > trait Config {
   > 	type C: PartialEq;
   > }
   >
   > // Foo implements [`PartialEq`] because `C` bounds [`PartialEq`].
   > // Otherwise compilation will fail with an output telling `c` doesn't implement [`PartialEq`].
   > #[derive(PartialEqNoBound)]
   > struct Foo<T: Config> {
   > 	c: T::C,
   > }
   > ```

   This is essentially a workaround for [this issue][derive-bounds-issue] and
   [this issue][where-clause-elaboration-issue], allowing one to only bound what
   is necessary when the compiler is unable to figure it out.

   Note the `#[scale_info(skip_type_params(Bound))]` on the above
   implementation - this attribute is used by the [`scale-info`][scale-info]
   crate to allow fine-grained control over which generic parameters are
   included in the generated type definition. This, in combination with
   [`bounds(..)`][scale-info-bounds], enables full control over the generated
   bounds for the trait implementations.

   > serde also uses the `bounds(..)` pattern, both at the
   > [field-level][serde-attr-bound] and the
   > [container-level][serde-container-attr-bound].

   In practice, this isn't typically an issue, since these types are only ever
   used in contexts where these constraints are upheld (i.e. when using a
   runtime/`<T as Config>::Type`). However, as we attempt to move towards
   decoupling our business logic from the pallet, this will require us to add
   (seemingly unnecessary) bounds to functions and types that make use of types
   defined in this way.

[dacp]:
  https://github.com/ComposableFi/composable/blob/d90581d9349eb088ca930cf971dc05a21dca56b7/code/parachain/frame/pablo/src/dual_asset_constant_product.rs#L28
  "DualAssetConstantProduct"
[convert]:
  https://paritytech.github.io/substrate/master/sp_runtime/traits/trait.Convert.html
  "Convert trait"
[fungibles]:
  https://paritytech.github.io/substrate/master/frame_support/traits/tokens/fungibles/index.html
  "Fungibles traits"
[no-bound-derives]:
  https://crates.parity.io/frame_support/index.html#derives
  "*NoBound derive macros"
[partial-eq-no-bound]:
  https://crates.parity.io/frame_support/derive.PartialEqNoBound.html
  "PartialEqNoBound"
[derive-bounds-issue]:
  https://github.com/rust-lang/rust/issues/26925
  "#[derive] sometimes uses incorrect bounds"
[where-clause-elaboration-issue]:
  https://github.com/rust-lang/rust/issues/20671
  "Where-clauses are only elaborated for supertraits"
[scale-info]: https://docs.rs/scale-info/latest/scale_info/ "scale_info crate"
[scale-info-bounds]:
  https://docs.rs/scale-info/latest/scale_info/#scale_infobounds
  "#[scale_info(bounds(..))]"
[serde-attr-bound]: https://serde.rs/attr-bound.html "Serde bound attribute"
[serde-container-attr-bound]:
  https://serde.rs/container-attrs.html#bound
  "Serde container-level bound attribute"
[test-externalities]:
  https://paritytech.github.io/substrate/master/sp_io/type.TestExternalities.html
  "Test Externalities"
[hooks-example]:
  ../code/parachain/frame/pablo/src/rfc_0015_example_hooks.rs
[explicit-example]:
  ../code/parachain/frame/pablo/src/rfc_0015_example_explicit.rs
