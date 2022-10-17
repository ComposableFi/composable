import { TokenId } from "tokens";

export const currencyIdToAssetMap: Record<string, TokenId | string[]> = {
  "1": "pica",
  "4": "ksm",
  "10": ["4", "1"]
};
