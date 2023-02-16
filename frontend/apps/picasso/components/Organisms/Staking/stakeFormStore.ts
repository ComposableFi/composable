import create from "zustand";
import { devtools, subscribeWithSelector } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import BigNumber from "bignumber.js";
import { useStore } from "@/stores/root";

type StakeFormActions = {
  setLockPeriod: (lockPeriod: string) => void;
  setAmount: (v: BigNumber) => void;
  setFormValidation: (v: boolean, message: string) => void;
};

type StakeFormData = {
  lockPeriod: string;
  amount: BigNumber;
  isFormValid: boolean;
  isFormDirty: boolean;
  validationMessage: string;
};

export const useStakeForm = create<StakeFormData & StakeFormActions>()(
  devtools(
    immer(
      subscribeWithSelector((set, get) => ({
        isFormDirty: false,
        isFormValid: true,
        lockPeriod: "0",
        validationMessage: "",
        amount: new BigNumber(0),
        setLockPeriod: (lockPeriod: string) => {
          set((state) => {
            state.lockPeriod = lockPeriod;
            state.isFormDirty = true;
          });
        },
        setFormValidation: (v: boolean, message: string) => {
          set((state) => {
            if (get().isFormDirty) {
              state.isFormValid = v;
              state.validationMessage = message;
            }
          });
        },
        setAmount: (amount) => {
          set((state) => {
            state.amount = amount;
            state.isFormDirty = true;
          });
        },
      }))
    )
  )
);

function isValidStakeForm(amount: BigNumber, lockPeriod: string) {
  const minStake = useStore.getState().rewardPools[1].minimumStakingAmount;

  const message = amount.lt(minStake)
    ? `Minimum staked amount is ${minStake}`
    : "Select a lock period";
  const isValid = !isNaN(Number(lockPeriod)) && amount.gte(minStake);
  return { message, isValid };
}

export const subscribeStakeFormValidation = () =>
  useStakeForm.subscribe(
    (state) => ({
      lockPeriod: state.lockPeriod,
      amount: state.amount,
    }),
    ({ lockPeriod, amount }) => {
      const { message, isValid } = isValidStakeForm(amount, lockPeriod);

      useStakeForm
        .getState()
        .setFormValidation(isValid, isValid ? "" : message);
    },
    {
      equalityFn: (a, b) =>
        a.amount.eq(b.amount) && a.lockPeriod === b.lockPeriod,
    }
  );
