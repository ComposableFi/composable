import { toChainIdUnit } from "shared";
import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types";
import { XcmVersionedMultiLocation } from "@acala-network/types/interfaces/types-lookup";
import { CurrencyId } from "defi-interfaces";
import BigNumber from "bignumber.js";

export const getXTokenTransferCallOriginPicasso = (
    api: ApiPromise,
    destination: XcmVersionedMultiLocation,
    currencyId: BigNumber,
    amount: u128,
    hasFeeToken: boolean,
    feeTokenId: BigNumber | null,
    destinationWeight: number = 9000000000
) => {
    let _currencyId = api.createType("CurrencyId", currencyId.toString());
    const destWeight = api.createType("u64", destinationWeight);

    const amountParams = hasFeeToken ? [
        [
            _currencyId,
            amount
        ],
        [
            api.createType("u128", feeTokenId?.toString()),
            api.createType("u128", toChainIdUnit(1).toString()),
        ]
    ] : _currencyId;

    return !hasFeeToken ? api.tx.xTokens.transfer(
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