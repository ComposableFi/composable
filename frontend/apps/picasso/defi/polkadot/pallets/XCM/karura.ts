import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types";
import { XcmVersionedMultiLocation } from "@acala-network/types/interfaces/types-lookup";
import { CurrencyId } from "defi-interfaces";

export const getParachainDestinationCallOriginKarura = (
    api: ApiPromise,
    destination: XcmVersionedMultiLocation,
    currencySymbol: string | null,
    amount: u128,
    destinationWeight: number = 20000000000000
) => {
    const _currencyId: CurrencyId = api.createType(
        "AcalaPrimitivesCurrencyCurrencyId",
        {
          Token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", currencySymbol),
        }
      )

    const destWeight = api.createType("u64", destinationWeight);

    return api.tx.xTokens.transfer(
        _currencyId,
        amount,
        destination,
        destWeight
    )
}