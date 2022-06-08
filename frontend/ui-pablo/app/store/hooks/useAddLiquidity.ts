import {
  Assets,
  AssetsValidForNow,
  getAsset,
  getAssetOnChainId,
} from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { getPairDecimals } from "@/defi/polkadot/utils";
import { setSelection, useAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID, DEFAULT_DECIMALS } from "@/updaters/constants";
import BigNumber from "bignumber.js";
import { useState, useMemo, useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { useLiquidityByPool } from "./useLiquidityByPool";

export const useAddLiquidity = () => {
  const {
    assetBalances,
    poolStats
  } = useStore();
  const [valid, setValid] = useState<boolean>(false);
  const { parachainApi } = useParachainApi("picasso");

  const {
    ui: { assetOne, assetTwo, assetOneAmount, assetTwoAmount },
    pool,
    findPoolManually
  } = useAddLiquiditySlice();

  const {
    tokenAmounts: {
      baseAmount,
      quoteAmount
    }
  } = useLiquidityByPool(pool)

  const assetOneAmountBn = useMemo(
    () => new BigNumber(assetOneAmount),
    [assetOneAmount]
  );
  const assetTwoAmountBn = useMemo(
    () => new BigNumber(assetTwoAmount),
    [assetTwoAmount]
  );

  const assetList1 = useMemo(() => {
    return Object.values(Assets)
      .filter((i) => {
        return AssetsValidForNow.includes(i.assetId) && i.assetId !== assetTwo;
      })
      .map((asset) => ({
        value: asset.assetId,
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [assetTwo]);

  const assetList2 = useMemo(() => {
    return Object.values(Assets)
      .filter((i) => {
        return AssetsValidForNow.includes(i.assetId) && i.assetId !== assetOne;
      })
      .map((asset) => ({
        value: asset.assetId,
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [assetOne]);

  const balanceOne = useMemo(() => {
    if (assetOne !== "none") {
      return new BigNumber(assetBalances[assetOne as AssetId].picasso);
    } else {
      return new BigNumber(0);
    }
  }, [assetOne, assetBalances]);

  const balanceTwo = useMemo(() => {
    if (assetTwo !== "none") {
      return new BigNumber(assetBalances[assetTwo as AssetId].picasso);
    } else {
      return new BigNumber(0);
    }
  }, [assetTwo, assetBalances]);

  const setAmount =
    (key: "assetOneAmount" | "assetTwoAmount") => (v: BigNumber) => {
      setSelection({ [key]: v.toString() });
    };

  const setToken = (key: "assetOne" | "assetTwo") => (v: AssetId) => {
    setSelection({ [key]: v });
  };

  const isValidToken1 = assetOne != "none";
  const isValidToken2 = assetTwo != "none";

  const needToSelectToken = () => {
    return !isValidToken1 && !isValidToken2;
  };

  const invalidTokenPair = () => {
    return (
      (!isValidToken1 && isValidToken2) || (isValidToken1 && !isValidToken2)
    );
  };

  const canSupply = () => {
    return assetOneAmountBn.lt(balanceOne) && assetTwoAmountBn.lt(balanceTwo);
  };

  useEffect(() => {
    setValid(true);
    assetOne == "none" && setValid(false);
    assetTwo == "none" && setValid(false);

    new BigNumber(0).eq(assetOneAmountBn) && setValid(false);
    new BigNumber(0).eq(assetTwoAmountBn) && setValid(false);

    balanceOne.lt(assetOneAmountBn) && setValid(false);
    balanceTwo.lt(assetTwoAmountBn) && setValid(false);

    pool && pool.poolId === -1 && setValid(false);
  }, [
    assetOne,
    assetTwo,
    assetOneAmountBn,
    assetTwoAmountBn,
    balanceOne,
    balanceTwo,
    pool,
  ]);

  const assetOneMeta = useMemo(() => {
    return assetOne === "none" ? null : getAsset(assetOne);
  }, [assetOne]);

  const assetTwoMeta = useMemo(() => {
    return assetTwo === "none" ? null : getAsset(assetTwo);
  }, [assetTwo]);

  const assetOneReserve = useMemo(() => {
    if (
      pool &&
      pool.poolId !== -1 &&
      poolStats[pool.poolId] &&
      assetOne !== "none"
    ) {
      const assetOneOnChainId = getAssetOnChainId(DEFAULT_NETWORK_ID, assetOne);
      if (assetOneOnChainId && assetOneOnChainId === pool.pair.base) {
        return new BigNumber(baseAmount);
      } else {
        return new BigNumber(quoteAmount);
      }
    } else {
      return new BigNumber(0);
    }
  }, [pool, poolStats, assetOne]);

  const assetTwoReserve = useMemo(() => {
    if (
      pool &&
      pool.poolId !== -1 &&
      poolStats[pool.poolId] &&
      assetTwo !== "none"
    ) {
      const assetTwoOnChainId = getAssetOnChainId(DEFAULT_NETWORK_ID, assetTwo);
      if (assetTwoOnChainId && assetTwoOnChainId === pool.pair.quote) {
        return new BigNumber(quoteAmount);
      } else {
        return new BigNumber(baseAmount);
      }
    } else {
      return new BigNumber(0);
    }
  }, [pool, poolStats, assetTwo]);

  const share = useMemo(() => {
    let netAum = new BigNumber(assetOneReserve).plus(assetTwoReserve);
    let netUser = new BigNumber(assetOneAmountBn).plus(assetTwoAmountBn);

    if (netAum.eq(0)) {
      return new BigNumber(100);
    } else {
      return new BigNumber(netUser)
        .div(new BigNumber(netAum).plus(netUser))
        .times(100);
    }
  }, [pool, assetOneAmountBn, assetTwoAmountBn]);

  const [lpReceiveAmount, setLpReceiveAmount] = useState(new BigNumber(0));

  useEffect(() => {
    if (parachainApi && assetOne !== "none" && assetTwo !== "none" && pool) {
      const { baseDecimals, quoteDecimals } = getPairDecimals(
        assetOne,
        assetTwo
      );

      let isReverse =
        pool.pair.base !== getAsset(assetOne).supportedNetwork.picasso;
      const bnBase = new BigNumber(
        isReverse ? assetTwoAmount : assetOneAmount
      ).times(isReverse ? quoteDecimals : baseDecimals);
      const bnQuote = new BigNumber(
        isReverse ? assetOneAmount : assetTwoAmount
      ).times(isReverse ? baseDecimals : quoteDecimals);

      if (bnBase.gte(0) && bnQuote.gte(0)) {
        (parachainApi.rpc as any).pablo
          .expectedLpTokensGivenLiquidity(
            parachainApi.createType("PalletPabloPoolId", pool.poolId),
            parachainApi.createType("CustomRpcBalance", bnBase.toString()),
            parachainApi.createType("CustomRpcBalance", bnBase.toString())
          )
          .then((expectedLP: any) => {
            setLpReceiveAmount(
              new BigNumber(expectedLP.toString()).div(DEFAULT_DECIMALS)
            );
          })
          .catch((err: any) => {
            console.log(err);
          });
      }
    }
  }, [parachainApi, assetOneAmount, assetTwoAmount]);

  return {
    assetOne,
    assetTwo,
    balanceOne,
    balanceTwo,
    assetOneAmountBn,
    assetTwoAmountBn,
    assetList1,
    assetList2,
    share,
    lpReceiveAmount,
    assetOneMeta,
    assetTwoMeta,
    valid,
    isValidToken1,
    isValidToken2,
    setValid,
    setToken,
    setAmount,
    needToSelectToken,
    invalidTokenPair,
    canSupply,
    findPoolManually
  };
};
