import create from "zustand";
import { TransactionSettings } from "@/defi/types";

export interface AppSettingsSlice {
    maxTolerance: number,
    minTolerance: number,
    maxDeadline: number,
    minDeadline: number,
    transactionSettings: TransactionSettings,
}

export const useAppSettingsSlice = create<AppSettingsSlice>(() => ({
    maxTolerance: 100,
    minTolerance: 0,
    maxDeadline: 9999,
    minDeadline: 0,
    transactionSettings: {
        tolerance: 0.1,
        deadline: 20,
    },
}));


export const setAppSetting = (setting: Partial<AppSettingsSlice>) =>
    useAppSettingsSlice.setState((state) => {
        state = { ...state, ...setting }
        return state;
    });

export const setTransactionSetting = (setting: AppSettingsSlice["transactionSettings"]) =>
    useAppSettingsSlice.setState((state) => {
        state = { ...state, transactionSettings: setting }
        return state;
    });