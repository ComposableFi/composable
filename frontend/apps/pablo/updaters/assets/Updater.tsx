import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useEffect } from "react";
import {
  picassoAssetsList,
  subscribeNativeBalance,
  subscribePicassoBalanceByAssetId,
} from "shared";
import { TokenId } from "tokens";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

const Updater = () => {
  const { substrateTokens, substrateBalances } = useStore();
  const { setTokenBalance } = substrateBalances;
  const { setTokens, tokens, hasFetchedTokens } = substrateTokens;
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const account = useSelectedAccount("picasso");

  useEffect(() => {
    if (parachainApi) {
      picassoAssetsList(parachainApi).then((list) => {
        setTokens({
          picasso: {
            list,
            api: parachainApi,
          },
        });
      });
    }
  }, [parachainApi, setTokens]);

  useEffect(() => {
    if (!parachainApi || !hasFetchedTokens || !account) return;
    const supportedTokens: TokenId[] = [];

    const allTokens = Object.entries(tokens);
    for (const [id, token] of allTokens) {
      if (token.isSupportedOn("picasso")) {
        supportedTokens.push(id as TokenId);
      }
    }

    let subscriptions: any[] = [];
    supportedTokens.forEach((tokenId) => {
      if (tokenId !== SUBSTRATE_NETWORKS.picasso.tokenId) {
        subscribePicassoBalanceByAssetId(
          parachainApi,
          account.address,
          tokens[tokenId].getPicassoAssetId(true) as BigNumber,
          tokens[tokenId].getDecimals("picasso") as number,
          (balance) => {
            setTokenBalance(tokenId, "picasso", balance.free, balance.locked);
          }
        ).then((sub) => {
          subscriptions.push(sub);
        });
      } else {
        subscribeNativeBalance(
          parachainApi,
          account.address,
          "picasso",
          (balance) => {
            setTokenBalance(tokenId, "picasso", balance.free, balance.locked);
          }
        ).then((sub) => {
          subscriptions.push(sub);
        });
      }
    });

    return function () {
      subscriptions.map((sub) => {
        sub?.();
      });
    };
  }, [
    parachainApi,
    substrateTokens,
    hasFetchedTokens,
    account,
    tokens,
    setTokenBalance,
  ]);

  return null;
};

export default Updater;
