## Automatically Sorted Vec as a storage item

In order to enable the functionality of having a sorted vector 
as a storage item. The sortedvec has been implemented.
This allows anyone to have a sorted accessable vector.


## Examples:   

See sortedvec.rs for the latest source codes and code samples

```rust
sortedvec! {    
                /// lookup by (amount, votes) keys 
                #[derive(Debug, Encode, Decode, TypeInfo)]//EncodeLike  
                pub struct FastMap {  
                                fn derive_key(val: &BribesStorage) -> (u32, u32) {
                                                (val.amount, val.votes) 
                                }    
                }  
}     

impl FastMap {                                                                         
                                                                                       
        // make it easier to add things                                                
        pub fn add(&mut self, amounts: u32, pid: u32, vots: u32) -> bool {             
                self.insert(BribesStorage { p_id: pid, amount: amounts, votes: vots });
                true                                                                   
        }                                                                              
                                                                                       
        pub fn new() -> FastMap {                                                      
                FastMap::default()                                                     
        }                                                                              
}                                                                                      


```



```rust 
pub use crate::sortedvec::FastMap;

#[pallet::storage]
#[pallet::getter(fn fast_vexc)] 
pub(super) type Fastvec<T: Config> = StorageValue<_, FastMap, ValueQuery>;

Fastvec::<T>::mutate(|a| a.add(1, 2, 3));

```


Links: 
https://docs.substrate.io/v3/runtime/storage/   


