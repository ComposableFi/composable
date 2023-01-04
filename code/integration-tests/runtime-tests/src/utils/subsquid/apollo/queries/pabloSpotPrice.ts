import gql from "../gql";

export type PabloSpotPrice = {
  pabloSpotPrice: {
    spotPrice: string;
  };
};

export const PABLO_SPOT_PRICE = gql`
    query pabloSpotPrice($baseAssetId: String!, $quoteAssetId: String!, $poolId: String!) {
        pabloSpotPrice(params: { baseAssetId: $baseAssetId, quoteAssetId: $quoteAssetId, poolId: $poolId }) {
            spotPrice
        }
    }
`;
