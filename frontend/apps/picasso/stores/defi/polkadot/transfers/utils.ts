import { TokenId } from "tokens";
import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";

export function getDefaultToken(tokenOptions: Array<TokenOption>): TokenId {
  const found = tokenOptions.find((token) => !token.disabled);
  if (found) {
    return found.tokenId;
  }

  return tokenOptions?.[0]?.tokenId;
}
