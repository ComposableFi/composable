import create from "zustand";
import { devtools, subscribeWithSelector } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import BigNumber from "bignumber.js";

type StakeFormActions = {
  setLockPeriod: (lockPeriod: string) => void;
  setAmount: (v: BigNumber) => void;
  setFormValidation: (v: boolean) => void;
};

type StakeFormData = {
  lockPeriod: string;
  amount: BigNumber;
  isFormValid: boolean;
  isFormDirty: boolean;
};

export const useStakeForm = create<StakeFormData & StakeFormActions>()(
  devtools(
    immer(
      subscribeWithSelector((set, get) => ({
        isFormDirty: false,
        isFormValid: true,
        lockPeriod: "",
        amount: new BigNumber(0),
        setLockPeriod: (lockPeriod: string) => {
          set((state) => {
            state.lockPeriod = lockPeriod;
            state.isFormDirty = true;
          });
        },
        setFormValidation: (v: boolean) => {
          set((state) => {
            if (get().isFormDirty) {
              state.isFormValid = v;
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

export const subscribeStakeFormValidation = () =>
  useStakeForm.subscribe(
    (state) => ({
      lockPeriod: state.lockPeriod,
      amount: state.amount,
    }),
    ({ lockPeriod, amount }) => {
      useStakeForm
        .getState()
        .setFormValidation(!isNaN(Number(lockPeriod)) && amount.gt(0));
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) =>
        a.amount.eq(b.amount) && a.lockPeriod === b.lockPeriod,
    }
  );
