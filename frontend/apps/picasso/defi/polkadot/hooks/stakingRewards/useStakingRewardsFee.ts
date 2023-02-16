import {
  useExecutor,
  usePicassoProvider,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import config from "@/constants/config";
import { useStore } from "@/stores/root";
import { useCallback, useEffect } from "react";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import * as TE from "fp-ts/TaskEither";
import { fromChainIdUnit } from "shared";
import { FEE_MULTIPLIER } from "shared/defi/constants";
import { subscribeStakingFee } from "@/stores/defi/polkadot/stakingRewards/subscribeStakingFee";
import BigNumber from "bignumber.js";

export const useStakingRewardsFee = () => {
  const { parachainApi } = usePicassoProvider();
  const executor = useExecutor();
  const account = useSelectedAccount(config.defaultNetworkId);
  const signer = useSigner();
  const pica = useStore(({ substrateTokens }) => substrateTokens.tokens.pica);
  const feeItem = useStore((store) => store.transfers.feeItem);

  const feeCall = useCallback(
    (balance: BigNumber, durationPreset: string) =>
      pipe(
        O.Do,
        O.bind("api", () => O.fromNullable(parachainApi)),
        O.bind("exec", () => O.fromNullable(executor)),
        O.bind("acc", () => O.fromNullable(account)),
        O.bind("sgn", () => O.fromNullable(signer)),
        TE.fromOption(() => new Error("Not enough parameters")),
        TE.chain(({ api, acc, exec, sgn }) =>
          TE.tryCatch(
            () =>
              exec.paymentInfo(
                api.tx.stakingRewards.stake(
                  pica.chainId.picasso?.toString() || "1",
                  fromChainIdUnit(
                    balance,
                    pica.decimals.picasso ?? 12
                  ).toString(),
                  api.createType("u64", durationPreset)
                ),
                acc.address,
                sgn
              ),
            () => new Error("Could not fetch stake fee")
          )
        ),
        TE.map((dispatchInfo) => {
          const ratio =
            useStore.getState().substrateTokens.tokens[feeItem].ratio.picasso;
          if (ratio) {
            const fee = fromChainIdUnit(
              dispatchInfo.partialFee.toString(),
              pica.decimals.picasso
            )
              .multipliedBy(FEE_MULTIPLIER)
              .multipliedBy(ratio.n)
              .div(ratio.d);
            return {
              assetId: feeItem,
              fee,
            };
          } else {
            return {
              assetId: feeItem,
              fee: fromChainIdUnit(
                dispatchInfo.partialFee.toString(),
                pica.decimals.picasso
              ).multipliedBy(FEE_MULTIPLIER),
            };
          }
        })
      ),
    [
      account,
      executor,
      feeItem,
      parachainApi,
      pica.chainId.picasso,
      pica.decimals.picasso,
      signer,
    ]
  );

  useEffect(() => subscribeStakingFee(feeCall), [feeCall]);
};
