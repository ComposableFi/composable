## TokenFactory

Token factory is used by the vault to create `IOUToken` and `ReceiptToken` contracts.

Users earn fees for providing liquidity in the vaults. There are 2 types of liquidity passive and active.

`ReceiptToken` is issued for passive liquidity.

Passive liquidity is entitled to receive fee earned by the system for transferring of tokens between layers. Users can
provide passive liquidity anytime they want and for any period.

`IOUToken` is issued for active liquidity. Active liquidity behaves as passive liquidity after the specified
active liquidity period is over.

Active liquidity is needed when the current liquidity in the vault is not enough to cover all the transfers. In this
case users can provide active liquidity which earns higher fee than passive liquidity. Active liquidity can be provided
only when the vault needs it and only for a specified period.

[home](/readme.md)
