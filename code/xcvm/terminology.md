# Terminology

We use a lot of different terms across these specifications, some of which can be very overloaded in meaning (a transaction referring to a transactional change to the state or a user fund transfer). We try to keep the same terminology consistent everywhere, so if you find a usage that does not coincide with the definition in this document, please open an issue or PR.

- `Transaction`: A reversible operation on a chain.
- `Transfer`: Changing the ownership of an asset (token, NFT, etc) from one account to another.
- `Identity`: Amn entity that has an address. Note that identities may not have a public/private key, as they can be contracts.
- `Cross Chain Transfer`: The bridging of funds between two chains.
- `Cross Chain Transaction`: Two or more transactions on two or more chains. Note that a cross-chain transaction is not `transactional`.
- `XCVM Transaction`: A cross-chain transaction defined as XCVM instructions, being handled by interpreters. Technically an XCVM transaction can be single chain only, although the use case for that seems non-existent.
- `Message Passing`: Sending bytes from one chain to another.
- `Event`: A message emitted by a contract/module/pallet during transaction execution.
- `XCVM Event`: An event emitted by part of the XCVM contracts.