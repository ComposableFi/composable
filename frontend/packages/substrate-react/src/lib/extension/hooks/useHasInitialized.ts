import { useSubstrateReact } from "../store/extension.slice";

export const useHasInitialized = () => useSubstrateReact().hasInitialized