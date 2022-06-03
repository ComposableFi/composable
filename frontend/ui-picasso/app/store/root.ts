import create, { GetState, State } from "zustand";
import { devtools, NamedSet } from "zustand/middleware";
import { createCommonSlice } from "./common";
import immer from "./middlewares/immer";
import { AppState, CustomStateCreator } from "./types";

export const createStore = <TState extends State>(
  storeCreator: CustomStateCreator<TState>
) => {
  return create(devtools(immer(storeCreator)));
};

export const useStore = createStore<AppState>(
  (set: NamedSet<any>, get: GetState<any>) => ({
    ...createCommonSlice(set, get),
  })
);
