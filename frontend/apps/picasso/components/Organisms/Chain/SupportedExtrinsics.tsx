import { ReactNode } from "react";
import { isPalletSupported } from "shared";
import { usePicassoProvider } from "substrate-react";

type Props = {
  pallet: string;
  children: ReactNode;
};

export function SupportedExtrinsics({ pallet, children }: Props) {
  const { parachainApi } = usePicassoProvider();
  return isPalletSupported(parachainApi)(pallet) ? children : null;
}
