import { FC, useCallback, useEffect } from "react";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import { useSnackbar } from "notistack";
import { stake } from "@/defi/polkadot/pallets/StakingRewards";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { Executor, useSigner } from "substrate-react";
import {
  subscribeStakeFormValidation,
  useStakeForm,
} from "@/components/Organisms/Staking/stakeFormStore";
import {
  getMaxDuration,
  getMinDuration,
  getOptions,
} from "@/components/Organisms/Staking/utils";
import { pipe } from "fp-ts/lib/function";
import * as O from "fp-ts/Option";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { StakeForm } from "@/components/Organisms/Staking/StakeForm";

type StakeTabContentProps = {
  executor: Executor;
  parachainApi: ApiPromise;
};

export const StakeTabContent: FC<StakeTabContentProps> = ({
  executor,
  parachainApi,
}) => {
  const { isFormValid, lockPeriod, setLockPeriod, amount, setAmount } =
    useStakeForm((state) => state);
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const { hasRewardPools, picaRewardPool, balance, pica } = useStakingRewards();

  const account = usePicassoAccount();
  const signer = useSigner();
  const options = getOptions(hasRewardPools, picaRewardPool);
  const minDuration = getMinDuration(hasRewardPools, picaRewardPool);
  const maxDuration = getMaxDuration(hasRewardPools, picaRewardPool);
  const handleStakeClick = useCallback(() => {
    pipe(
      O.Do,
      O.bind("assetId", () => O.fromNullable(pica.chainId.picasso?.toNumber())),
      O.map(({ assetId }) =>
        stake({
          executor,
          parachainApi,
          account,
          assetId,
          lockablePICA: amount,
          lockPeriod,
          enqueueSnackbar,
          closeSnackbar,
          signer,
        })
      )
    );
  }, [
    account,
    amount,
    pica,
    closeSnackbar,
    enqueueSnackbar,
    executor,
    lockPeriod,
    parachainApi,
    signer,
  ]);

  const setValidation = () => {};

  useEffect(() => {
    return subscribeStakeFormValidation();
  }, []);
  return (
    <StakeForm
      amount={balance}
      pica={pica}
      valid={setValidation}
      setter={setAmount}
      value={amount}
      options={options}
      picaRewardPool={picaRewardPool}
      duration={lockPeriod}
      onNone={() => new BigNumber(0)}
      onSome={(x) => x}
      hasRewardPools={hasRewardPools}
      min={minDuration}
      max={maxDuration}
      onChange={(_, value) => setLockPeriod(value.toString())}
      onClick={() => handleStakeClick()}
      formValid={isFormValid}
    />
  );
};
