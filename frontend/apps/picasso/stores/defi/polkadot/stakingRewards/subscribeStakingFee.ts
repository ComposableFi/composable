import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import BigNumber from "bignumber.js";
import * as TE from "fp-ts/lib/TaskEither";
import * as E from "fp-ts/lib/Either";
import { useStakeForm } from "@/components/Organisms/Staking/stakeFormStore";
import { useStore } from "@/stores/root";
import { flow } from "fp-ts/lib/function";

export function subscribeStakingFee(
  call: (
    balance: BigNumber,
    durationPreset: string
  ) => TE.TaskEither<
    Error,
    {
      assetId: TokenMetadata["id"];
      fee: BigNumber;
    }
  >
) {
  return useStore.subscribe(
    (state) => ({
      balance: state.substrateBalances.balances.picasso.pica.free,
    }),
    ({ balance }) => {
      const durationPreset = useStakeForm.getState().lockPeriod;

      call(balance, durationPreset)().then(
        flow(
          E.fold(
            () => {},
            (feeRecord) => {
              console.log("Storing fee");
              useStore.setState((state) => {
                state.stakingFee = feeRecord;
              });
            }
          )
        )
      );
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => a.balance.eq(b.balance),
    }
  );
}
