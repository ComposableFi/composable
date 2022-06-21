import {
  Assets,
  AssetsValidForNow,
  getAssetOnChainId,
} from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { setSelection, useAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID, DEFAULT_DECIMALS } from "@/defi/utils/constants";
import BigNumber from "bignumber.js";
import { useState, useMemo, useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { useLiquidityByPool } from "./useLiquidityByPool";
import { useAssetBalance } from "../assets/hooks";
import { toChainUnits } from "@/defi/utils";

export const useAddLiquidity = () => {
  const {
    supportedAssets
  } = useStore();
  const [valid, setValid] = useState<boolean>(false);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const {
    ui: { assetOne, assetTwo, assetOneAmount, assetTwoAmount },
    pool,
    findPoolManually
  } = useAddLiquiditySlice();

  const _assetOne = useMemo(() => {
    return supportedAssets.find(i => i.network[DEFAULT_NETWORK_ID] === assetOne)
  }, [supportedAssets, assetOne]);

  const _assetTwo = useMemo(() => {
    return supportedAssets.find(i => i.network[DEFAULT_NETWORK_ID] === assetTwo)
  }, [supportedAssets, assetTwo]);

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
    return supportedAssets
      .filter((i) => {
        if (assetTwo === "none") return true;
        if (assetTwo === i.network[DEFAULT_NETWORK_ID]) return false;
        return true;
      })
      .map((asset) => ({
        value: asset.network[DEFAULT_NETWORK_ID],
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [assetTwo, supportedAssets]);

  const assetList2 = useMemo(() => {
    return supportedAssets
      .filter((i) => {
        if (assetOne === "none") return true;
        if (assetOne === i.network[DEFAULT_NETWORK_ID]) return false;
        return true;
      })
      .map((asset) => ({
        value: asset.network[DEFAULT_NETWORK_ID],
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [assetOne, supportedAssets]);

  const balanceOne = useAssetBalance(DEFAULT_NETWORK_ID, assetOne);
  const balanceTwo = useAssetBalance(DEFAULT_NETWORK_ID, assetTwo);

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

  const share = useMemo(() => {
    let netAum = new BigNumber(baseAmount).plus(quoteAmount);
    let netUser = new BigNumber(assetOneAmountBn).plus(assetTwoAmountBn);

    if (netAum.eq(0)) {
      return new BigNumber(100);
    } else {
      return new BigNumber(netUser)
        .div(new BigNumber(netAum).plus(netUser))
        .times(100);
    }
  }, [
    baseAmount,
    quoteAmount,
    assetOneAmountBn,
    assetTwoAmountBn
  ]);

  const [lpReceiveAmount, setLpReceiveAmount] = useState(new BigNumber(0));

  useEffect(() => {
    if (parachainApi && assetOne !== "none" && assetTwo !== "none" && pool) {
      let isReverse = pool.pair.base.toString() !== assetOne;
      const bnBase = toChainUnits(isReverse ? assetTwoAmount : assetOneAmount)
      const bnQuote = toChainUnits(isReverse ? assetOneAmount : assetTwoAmount);

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
  }, [parachainApi, assetOneAmount, assetTwoAmount, assetOne, assetTwo, pool]);

  return {
    assetOne: _assetOne,
    assetTwo: _assetTwo,
    balanceOne,
    balanceTwo,
    assetOneAmountBn,
    assetTwoAmountBn,
    assetList1,
    assetList2,
    share,
    lpReceiveAmount,
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
