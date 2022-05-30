import { toHexString } from "./hexStrings";

export const crowdLoanSignableMessage = (address: any) =>
  `picasso-${toHexString(address)}`;
