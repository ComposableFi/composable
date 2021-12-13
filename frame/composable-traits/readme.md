# Composable Traits   

#### Composable-traits allow us to write simple interfaces to thirdparty pallet functions. 


In order to have a clean source code and avoid adding unnecessary frame Config traits additions(example: `pub trait Config: frame_system::Config + thirdpartycrate::Config`), composable-traits comes to the rescue.  
Using traits we can easily import and use functions and logic from thirdparty frames without importing and configuring the thirdparty frame 
functions in lib.rs.

The logic for this is pretty simple, all we need to do is to create a function in our trait file that returns the same output as the original function. 
After we have created a function inside our custom trait that returns the same output, we can simply import it from composable_traits( ```rust use composable_traits::newtrait::Ourtrait ``` ) and call it with a ```rust T::Ourtrait::customfunction() ```.





## Code Examples:  

### [CurrencyFactory from currencies.rs](https://github.com/ComposableFi/composable/blob/main/frame/composable-traits/src/currency.rs#L25)     

### [asset_id from Vault](https://github.com/ComposableFi/composable/blob/main/frame/composable-traits/src/vault.rs#L75)      

### [get_all_markets from Lending](https://github.com/ComposableFi/composable/blob/main/frame/composable-traits/src/lending.rs#L118)   





## Links     
https://docs.substrate.io/v3/runtime/pallet-coupling/   
https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait    


