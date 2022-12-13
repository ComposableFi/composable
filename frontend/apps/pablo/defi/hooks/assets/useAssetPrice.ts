import { useEffect, useState } from "react";
import { LiquidityProviderToken, Asset } from "shared";
import { useAssetIdOraclePrice } from "./useAssetIdOraclePrice";
import { useLpTokenPrice } from "./useLpTokenPrice";
import BigNumber from "bignumber.js";

export function useAssetPrice(
    asset: Asset | LiquidityProviderToken | undefined
): BigNumber {
    const priceFromApollo = useAssetIdOraclePrice(
        asset?.getPicassoAssetId() as string
    );
    const lpTokenPrice = useLpTokenPrice(
        asset instanceof LiquidityProviderToken ? asset : undefined
    )

    const [price, setPrice] = useState(new BigNumber(0));
    useEffect(() => {
        if (asset instanceof LiquidityProviderToken) {
            setPrice(lpTokenPrice);
            return;
        }

        if (asset instanceof Asset) {
            setPrice(priceFromApollo);
            return;
        }
    }, [asset, lpTokenPrice, priceFromApollo]);

    return price;
}