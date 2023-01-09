import React from "react";
import { DotsamaContext } from "../DotSamaContext";
import { ParachainId } from "shared";

export const useParachainApi = (parachainId: ParachainId) => {
  const { parachainProviders } = React.useContext(DotsamaContext);
  return parachainProviders[parachainId];
};
