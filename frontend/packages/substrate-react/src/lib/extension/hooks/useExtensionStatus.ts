import { useSubstrateReact } from "../store/extension.slice";

export const useExtensionStatus = () => useSubstrateReact().extensionStatus;