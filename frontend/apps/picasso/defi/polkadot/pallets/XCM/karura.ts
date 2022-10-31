import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types";
import { XcmVersionedMultiLocation } from "@acala-network/types/interfaces/types-lookup";
import { CurrencyId } from "defi-interfaces";

/**
 * Make a Transfer call
 * on Karura via xTokens Pallet
 * @param api Connected to Picasso Chain
 * @param destination XcmVersionedMultiLocation => Parachain or RelayChain
 * @param currencySymbol Assets can be identified by their symbols on Karura
 * @param amount Amount of tokens
 * @param destinationWeight Need to confirm (hardcoded so far)
 * @returns XCM Transfer Call
 */
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
      Token: api.createType(
        "AcalaPrimitivesCurrencyTokenSymbol",
        currencySymbol
      ),
    }
  );

  const destWeight = api.createType("u64", destinationWeight);

  return api.tx.xTokens.transfer(_currencyId, amount, destination, destWeight);
};
