# Overview
Pallet-NFT provides basic non-fungible token functionality. 
This is especially necessary due to the various forms that NFTs can present as, each with their own unique traits.
In the following we will do a quick recap of what makes NFTs special followed by how that is reflected in Frame-NFT's code.
And finally, we will discuss two use cases.

---

# Non-Fungible Tokens
NFTs (Non-Fungible-Tokens) have a few characteristics setting them apart from fungible ones like currency, precious metals, or carbon credits:
* Non - Interchangeable 
* Specific creator and owner
* Uniquely Identifiable
* Provide complete history of their creation and trades

They represent ownership of a unique tangible or intangible asset, and record its history on the blockchain.

---

# Workflow
With the nature of NFTs in mind, it becomes imperative to have a straightforward implementation aiming to accommodate their traits.
We achieve this by having multiple methods to define distinct types and characteristics in an asset.
Ownership being a core concept in NFTs makes the presentation and trading of ownership a vital part of how we interact with them.
This makes storing and searching for specific information an essential part of this framework.
To that end, we leverage BTreeMaps as they reference data stored in them using BTreeSets consisting of two vectors.
Safety is key when it comes to processing of data, which is why we use Blake2-128Concat to hash all inputs and storage.

---

# Use Cases
Pallet-NFT is a module for integrators to make their own NFT applications.
Let's walk through at least two possible applications. One will be an NFT-Art-Gallery and the other a ticket booth.

### Art Gallery
An artist would mint their artwork into a wallet. 
They proceed to give each artwork their respective traits, for this example let's assume we have set names and prices.
Now we are only a few steps away from our first NFT-based art sale.

Taking advantage of the frameworks complete set of tools, an artist could:
* Host their own private Gallery of minted artwork
* Make their artwork searchable via recurring traits (style, color palette, materials, history)
* Transfer ownership of an art piece to a buyer

### Ticket booth
Moving on to the ticket booth
An organizer could mint a number of NFTs representing access to an event. 
By defining traits like seat numbers or levels of access, we could streamline the planning process of an event.
With each ticket having a unique digital signature and tracking its own history on the blockchain, we can make sure:
* The ticket is valid and was created by a licensed party
* If the ticket changed hands, and to whom
* To what and where the attendee has access to 
