## How to use the sudo god-mode in order to make privileged calls:   

Substrate allows us to easily make privileged calls and actions with the built-in function:   

*ensure_root(origin);*

Thanks to these functions, we are able to perform privledged calls from various accounts. Such as
upgrading the current blockchain. Only accounts that has the correct keys can execute functions that only  
allow Root origin to call them, meaning normal accounts can not execute them.  




Source code: https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/dispatch.rs#L24     


That can easily be imported from the default frame lib:

```rust
use frame_system::ensure_root;

```


In substrate the origin functions are used to define where the function is being called from and what privileges the call will have. You can choose between None, Signed and Root origin modes, root being the highest in the chain.   

Checking privledges of the origin with substrate:   
*  ensure_root, make sure the function is being used by someone we trust     
*  ensure_none, check if origin is none   
*  ensure_signed, make sure its signed    


## Examples:  
In order to use ensure_root we need to store the account key that is allowed to use
the privileged functions and then execute the ensure_root command.

```rust

use frame_system::ensure_root;

...
pub trait Config: frame_system::Config {}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

	#[weight = 0]
        pub fn privileged_function(origin) -> dispatch::DispatchResult {
            let rootman = ensure_root(origin)?;
	    let reward: u32 = 1337;
	    Self::deposit_event(Event::Locked(rootman, reward));
            Ok(())
        }
    }
}
...
```    

In this example, we create a privileged function and gives the sender/caller/origin root
privledges using the *ensure_root(origin)* function. Then we give the origin a reward of 1337 units.


Or if we want to write write functions that modify the heap(the programs allocated memory) we can do that:

```rust

fn modify_heap(origin: OriginFor<T>, magicfluff: u64){
	ensure_root(origin)?;
	storage::unhashed::put_raw(well_known_keys::HEAP_PAGES, &magicfluff.encode());

}


```

Or if we want to write a simple function for testing that displays a message if its  
used by a privledged account

```rust   

pub fn root_print(origin: OriginFor<T>) -> DispatchResult {   // Let's create a simple function that displays a message when its executed correctly, so we want to have the origin as input
	ensure_root(origin)?;// We want to check if the origin is Root
	info!("root_print has been executed by a root user!");// If all is well, we display a log message 
	Ok(())

}

```

## Protecting sudo functions   
In order to write safe functions that can execute privledged functions such as upgrading
our chain, we want to add a check, that verifies that the person using the function is allowed to do so.
This we can do with the *ensure_signed*(and scream if it fails) function from the frame_system library.  
We can simply do this by using the ensured_signed function then check the key and throw an error with  
an ensure! check.

Like this:   

```rust
use frame_system::ensure_signed;
...

fn powerfunction(origin: OriginFor<T>) {

  let sender = ensure_signed(origin)?; //Check if the sender has 
  ensure!(sender == Self::key(), Error::RequireSudo); // Verify that the function is done by someone holding a key that we are aware of and have verified. If this is not the case, we throw an error

  ... // do privledged stuff here

}

```


### References:  
https://substrate.dev/rustdocs/v3.0.0/frame_system/struct.EnsureRoot.html
https://substrate.dev/rustdocs/v3.0.0/frame_system/struct.EnsureSigned.html
https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/dispatch.rs#L24
https://github.com/ComposableFi/composable-node-beta/blob/main/pallets/oracle/src/lib.rs#L276
https://github.com/ComposableFi/composable-node-beta/blob/oracle/runtime/src/lib.rs#L312
https://www.shawntabrizi.com/substrate/the-sudo-story-in-substrate/
https://substrate.dev/docs/en/knowledgebase/runtime/origin
https://substrate.dev/docs/en/tutorials/forkless-upgrade/sudo-upgrade
https://github.com/paritytech/substrate/tree/master/frame/sudo


