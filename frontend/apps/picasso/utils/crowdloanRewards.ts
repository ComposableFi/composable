import { toHexString } from "shared";

export const crowdLoanSignableMessage = (address: any) =>
  `picasso-${toHexString(address)}`;
