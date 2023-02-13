import { useCallback, useEffect } from "react";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import { useSnackbar } from "notistack";
import { stake } from "@/defi/polkadot/pallets/StakingRewards";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { useExecutor, usePicassoProvider, useSigner } from "substrate-react";
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
import { StakeForm } from "@/components/Organisms/Staking/StakeForm";

export const StakeTabContent = () => {
  const { isFormValid, lockPeriod, setLockPeriod, amount, setAmount } =
    useStakeForm((state) => state);
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const { hasRewardPools, picaRewardPool, balance, pica } = useStakingRewards();
  const executor = useExecutor();
  const { parachainApi } = usePicassoProvider();
  const account = usePicassoAccount();
  const signer = useSigner();
  const options = getOptions(hasRewardPools, picaRewardPool);
  const minDuration = getMinDuration(hasRewardPools, picaRewardPool);
  const maxDuration = getMaxDuration(hasRewardPools, picaRewardPool);
  const handleStakeClick = useCallback(() => {
    pipe(
      O.Do,
      O.bind("assetId", () => O.fromNullable(pica.chainId.picasso?.toNumber())),
      O.bind("api", () => O.fromNullable(parachainApi)),
      O.bind("exec", () => O.fromNullable(executor)),
      O.map(({ assetId, api, exec }) =>
        stake({
          executor: exec,
          parachainApi: api,
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

  const setValidation = () => {}; // TODO: Implement validation for this or remove

  useEffect(() => subscribeStakeFormValidation(), []);

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
      hasRewardPools={hasRewardPools}
      min={minDuration}
      max={maxDuration}
      onChange={(_, value) => setLockPeriod(value.toString())}
      onClick={() => handleStakeClick()}
      formValid={isFormValid}
    />
  );
};
