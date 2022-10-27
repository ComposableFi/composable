import { toChainIdUnit } from "shared";
import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types";
import { XcmVersionedMultiLocation } from "@acala-network/types/interfaces/types-lookup";
import { CurrencyId } from "defi-interfaces";
import BigNumber from "bignumber.js";

/**
 * Make a Transfer or a MultiCurrencyTransfer call
 * on Picasso via xTokens Pallet
 * @param api Connected to Picasso Chain
 * @param destination XcmVersionedMultiLocation => Parachain or RelayChain
 * @param currencyId Asset Id of currency that needs to be transferred
 * @param amount Amount of tokens
 * @param hasFeeToken BYOG (need to validate and confirm)
 * @param feeTokenId Asset Id of token that will be used as fee
 * @param destinationWeight Need to confirm (hardcoded so far)
 * @returns XCM Transfer Call
 */
export const getXTokenTransferCallOriginPicasso = (
    api: ApiPromise,
    destination: XcmVersionedMultiLocation,
    currencyId: BigNumber,
    amount: u128,
    /**
     * Token Id is BigNumber because
     * Assets are identified by numeric
     * id on dali
     */
    feeTokenId: BigNumber | null,
    destinationWeight: number = 9000000000
) => {
    let _currencyId = api.createType("CurrencyId", currencyId.toString());
    const destWeight = api.createType("u64", destinationWeight);

    const amountParams = feeTokenId ? [
        [
            _currencyId,
            amount
        ],
        [
            api.createType("u128", feeTokenId?.toString()),
            api.createType("u128", toChainIdUnit(1).toString()),
        ]
    ] : _currencyId;

    return !feeTokenId ? api.tx.xTokens.transfer(
        amountParams as CurrencyId,
        amount,
        destination,
        destWeight
    ) : api.tx.xTokens.transferMulticurrencies(
        amountParams as [u128, u128][],
        api.createType("u32", 1),
        destination,
        destWeight
    )
}