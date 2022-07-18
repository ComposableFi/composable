import { TokenId } from "@/defi/Tokens";

export const currencyIdToAssetMap: Record<string, TokenId | string[]> = {
  "1": "pica",
  "4": "ksm",
  "10": ["4", "1"],
  "11": "chaos",
};
