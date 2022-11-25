import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { pipe } from "fp-ts/function";
import { option, readonlyArray } from "fp-ts";
import { ReactNode } from "react";
import { isPalletSupported } from "shared";

type Props = {
  pallet: string;
  children: ReactNode;
};

export function SupportedExtrinsics({ pallet, children }: Props) {
  const { parachainApi } = usePicassoProvider();
  return isPalletSupported(parachainApi)(pallet) ? children : null;
}
