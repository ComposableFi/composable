import remove_liquidity_formStories from "@/../storybook/stories/organisms/remove_liquidity_form.stories";
import { Option } from "@/components/types";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { DEFAULT_NETWORK_ID, fetchSpotPrice, isValidAssetPair, uniswapCalculator } from "@/defi/utils";
import { useAppSelector } from "@/hooks/store";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance, useUSDPriceByAssetId } from "@/store/assets/hooks";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js"
import { useSnackbar } from "notistack";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";

type MockedAssetOption = MockedAsset & Option

export function useSwaps(): {
    balance1: BigNumber;
    balance2: BigNumber;
    changeAsset: (side: "base" | "quote", asset: string | "none") => void;
    selectedAssetOneId: string | "none";
    selectedAssetTwoId: string | "none";
    selectedAssetOne: MockedAsset | undefined;
    selectedAssetTwo: MockedAsset | undefined;
    assetListOne: MockedAssetOption[];
    assetListTwo: MockedAssetOption[];
    assetOneAmount: BigNumber;
    assetTwoAmount: BigNumber;
    dexRoute: BigNumber | null;
    onChangeTokenAmount: (sideChange: "base" | "quote", amount: BigNumber) => void;
    updateSpotPrice: () => void;
    pabloPool: ConstantProductPool | StableSwapPool | undefined;
    minimumReceived: BigNumber;
    slippageAmount: BigNumber;
    feeCharged: BigNumber;
    spotPrice: BigNumber;
    valid: boolean;
    asset1PriceUsd: BigNumber;
    asset2PriceUsd: BigNumber;
    setAssetOneInputValid: (validity: boolean) => void;
    setAssetTwoInputValid: (validity: boolean) => void;
    assetOneInputValid: boolean;
    assetTwoInputValid: boolean;
} {
    const slippage = useAppSelector(
        (state) => state.settings.transactionSettings.tolerance
    );

    const [assetOneInputValid, setAssetOneInputValid] = useState(true);
    const [assetTwoInputValid, setAssetTwoInputValid] = useState(true);
    const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
    const { enqueueSnackbar } = useSnackbar();

    const { swaps, supportedAssets, pools: { constantProductPools, stableSwapPools } } = useStore();
    const {
        spotPrice,
        selectedAssets,
        selectedPool,
        setSpotPrice,
        setSelectedAsset,
        setSelectedPool,
        resetSwaps
    } = swaps;

    useEffect(() => {
        return () => {
            resetSwaps();
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    const {
        selectedAssetOneId,
        selectedAssetTwoId
    } = useMemo(() => {
        return {
            selectedAssetOneId: selectedAssets.quote,
            selectedAssetTwoId: selectedAssets.base
        }
    }, [selectedAssets])

    const setSelectedAssetOne = (id: string | "none") => {
        setSelectedAsset(id, "quote");
    }

    const setSelectedAssetTwo = (id: string | "none") => {
        setSelectedAsset(id, "base");
    }
    
    const asset1PriceUsd = useUSDPriceByAssetId(selectedAssetOneId);
    const asset2PriceUsd = useUSDPriceByAssetId(selectedAssetTwoId);
    
    const selectedAssetOne = useMemo(() => {return supportedAssets.find(asset => asset.network[DEFAULT_NETWORK_ID] === selectedAssetOneId)}, [supportedAssets, selectedAssetOneId])
    const selectedAssetTwo = useMemo(() => {return supportedAssets.find(asset => asset.network[DEFAULT_NETWORK_ID] === selectedAssetTwoId)}, [supportedAssets, selectedAssetTwoId])
    
    const assetListOne = useMemo(() => {
        return supportedAssets.filter(asset => {
            if (selectedAssetTwoId === "none") return true;
            if (selectedAssetTwoId === asset.network[DEFAULT_NETWORK_ID]) return false;
            return true;
        }).map(i => {return { ...i, value: i.network[DEFAULT_NETWORK_ID], label: i.symbol }})
    }, [selectedAssetTwoId, supportedAssets]);
    const assetListTwo = useMemo(() => {
        return supportedAssets.filter(asset => {
            if (selectedAssetOneId === "none") return true;
            if (selectedAssetOneId === asset.network[DEFAULT_NETWORK_ID]) return false;
            return true;
        }).map(i => {return { ...i, value: i.network[DEFAULT_NETWORK_ID], label: i.symbol }})
    }, [selectedAssetOneId, supportedAssets]);

    const balance1 = useAssetBalance(DEFAULT_NETWORK_ID, selectedAssetOneId);
    const balance2 = useAssetBalance(DEFAULT_NETWORK_ID, selectedAssetTwoId);

    const fetchDexRoute = useCallback(async (): Promise<BigNumber | null> => {
        if (parachainApi && isValidAssetPair(selectedAssetOneId, selectedAssetTwoId)) {
            const routePromises = [
                parachainApi.query.dexRouter.dexRoutes(selectedAssetOneId, selectedAssetTwoId),
                parachainApi.query.dexRouter.dexRoutes(selectedAssetTwoId, selectedAssetOneId),
            ];
            const dexRoutes = await Promise.all(routePromises);
            const [straightRouteResponse, inverseRouteResponse] = dexRoutes;
            let straightRoute = straightRouteResponse.toJSON();
            let inverseRoute = inverseRouteResponse.toJSON();

            let dexRoute: any = null;
            if (!!straightRoute) dexRoute = straightRoute;
            if (!!inverseRoute) dexRoute = inverseRoute;

            if (dexRoute && dexRoute.direct) {
                return new BigNumber(dexRoute.direct[0]);
            } else {
                return dexRoute;
            }
        }
        return null;
    }, [selectedAssetOneId, selectedAssetTwoId, parachainApi]);

    const [dexRoute, setDexRoute] = useState<BigNumber | null>(null);
    useAsyncEffect(async () => {
        const dexRoute = await fetchDexRoute();
        setDexRoute(dexRoute);
    }, [fetchDexRoute])

    useEffect(() => {
        if (!dexRoute) return undefined;

        const verifiedConstantProductPools = constantProductPools.verified;
        const verifiedStableSwapPools = stableSwapPools.verified;

        let lpToTrade: StableSwapPool | ConstantProductPool | undefined = undefined;
        lpToTrade = verifiedConstantProductPools.find(i => i.poolId === dexRoute.toNumber());
        if (!lpToTrade)
        lpToTrade = verifiedStableSwapPools.find(i => i.poolId === dexRoute.toNumber());

        setSelectedPool(lpToTrade)
    }, [dexRoute, constantProductPools, stableSwapPools, setSelectedPool]);

    const updateSpotPrice = useCallback(async () => {
        if (parachainApi && selectedPool) {
            const { base, quote } = selectedPool.pair;
            const isInverse = selectedAssetOneId === base.toString();
            let pair = { base: base.toString(), quote: quote.toString() };
            const spotPrice = await fetchSpotPrice(parachainApi, pair, selectedPool.poolId);
            if (isInverse) {
                setSpotPrice(new BigNumber(1).div(spotPrice).dp(0))
            } else {
                setSpotPrice(spotPrice.dp(0))
            }
        } else {
            setSpotPrice(new BigNumber(0))
        }
    }, [parachainApi, selectedPool, selectedAssetOneId, setSpotPrice]);

    useEffect(() => {
        if (selectedPool) {
            updateSpotPrice();
        }
    }, [selectedPool, updateSpotPrice]);

    const [tokenAmounts, setTokenAmounts] = useState({
        assetOneAmount: new BigNumber(0),
        assetTwoAmount: new BigNumber(0)
    });

    const [minimumReceived, setMinimumReceived] = useState(new BigNumber(0));
    const [slippageAmount, setSlippageAmount] = useState(new BigNumber(0));
    const [feeCharged, setFeeCharged] = useState(new BigNumber(0));

    const resetTokenAmounts = () => {
        setTokenAmounts(amounts => {
            return {
                assetOneAmount: new BigNumber(0),
                assetTwoAmount: new BigNumber(0),
            }
        });
    }

    const updateTokenAmount = async (changedSide: "base" | "quote", amount: BigNumber) => {
        if (parachainApi && selectedPool && isValidAssetPair(selectedAssetOneId, selectedAssetTwoId)) {
            const { base, quote } = selectedPool.pair;
            const { feeRate } = selectedPool.feeConfig;
            let feePercentage = new BigNumber(feeRate).toNumber();
            const isInverse = selectedAssetOneId === base.toString();
            let pair = { base: base.toString(), quote: quote.toString() };

            const oneBaseInQuote = await fetchSpotPrice(parachainApi, pair, selectedPool.poolId);
            const {
              minReceive,
              tokenOutAmount,
              feeChargedAmount,
              slippageAmount
            } = uniswapCalculator(changedSide, isInverse, amount, oneBaseInQuote, slippage, feePercentage);

            setTokenAmounts(amounts => {
                return {
                    assetOneAmount: changedSide === "base" ? amount : tokenOutAmount,
                    assetTwoAmount: changedSide === "quote" ? amount : tokenOutAmount,
                }
            });
            setMinimumReceived(minReceive);
            setFeeCharged(feeChargedAmount);
            setSlippageAmount(slippageAmount);
        } else {
            resetTokenAmounts();
            console.error(`Registered Pool not found`)
            enqueueSnackbar(`Registered Pool not found`)
        }
    }

    const onChangeTokenAmount = updateTokenAmount

    const changeAsset = (changedSide: "quote" | "base", tokenId: string | "none") => {
        changedSide === "quote" ? setSelectedAssetOne(tokenId) : setSelectedAssetTwo(tokenId)
        resetTokenAmounts();
    }

    const {
        assetOneAmount, assetTwoAmount
    } = tokenAmounts;

    const valid = dexRoute !== null && assetOneInputValid && assetTwoInputValid && !!selectedPool

    return {
        balance1,
        balance2,
        changeAsset,
        selectedAssetOneId,
        selectedAssetTwoId,
        selectedAssetOne,
        selectedAssetTwo,
        assetListOne,
        assetListTwo,
        onChangeTokenAmount,
        updateSpotPrice,
        assetOneAmount,
        assetTwoAmount,
        dexRoute,
        pabloPool: selectedPool,
        minimumReceived,
        slippageAmount,
        feeCharged,
        valid,
        spotPrice,
        asset1PriceUsd,
        asset2PriceUsd,
        setAssetOneInputValid,
        setAssetTwoInputValid,
        assetOneInputValid,
        assetTwoInputValid
    }
}