## ICS_026 Routing

The ICS routing specification enables ibc application protocols to receive packets after the core ibc protocol has verified  
the validity of the packet.

The [`Ics26Context`]() trait encapsulates the requirements for the router.  

As per previous context traits, the `Ics26Context` trait should be implemented for the Context object.  
`Ics26Context` has an associated type `Router` that accepts a type that implements the [`Router`]() trait.

It is recommended that the router use statically defined port and module Ids