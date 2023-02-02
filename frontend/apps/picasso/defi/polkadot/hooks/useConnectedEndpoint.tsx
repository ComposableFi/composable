import { usePicassoProvider } from "substrate-react";
import { useMemo } from "react";

export const useConnectedEndpoint = () =>  {
  const {parachainApi} = usePicassoProvider();

  return useMemo(() => {
    if (!parachainApi) return "";

    if (parachainApi.runtimeChain.toHuman() === "Picasso") return ""; // Only show runtime when connected to other endpoints.
    return parachainApi.runtimeChain.toHuman() + " " + parachainApi.runtimeVersion.specName.toString() + "/" +parachainApi.runtimeVersion.specVersion;
  }, [parachainApi])
}
