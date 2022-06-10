import produce, { Draft } from "immer";
import { State } from "zustand";

import { CustomStateCreator } from "../types";

const immer =
  <T extends State>(config: CustomStateCreator<T>): CustomStateCreator<T> =>
  (set, get, api) =>
    config(
      (partial, replace, name) => {
        const isFunction = typeof partial === "function";
        const functionName = isFunction && partial?.name;
        const nextState = isFunction
          ? produce(partial as (state: Draft<T>) => T)
          : (partial as T);
        return set(nextState, replace, name || functionName || "anonymous");
      },
      get,
      api
    );

export default immer;
