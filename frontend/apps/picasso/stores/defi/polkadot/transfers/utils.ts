import { TokenOption } from "@/stores/defi/polkadot/transfers/transfers";
import { AssetId } from "@/defi/polkadot/types";

export function getDefaultToken(tokenOptions: Array<TokenOption>): AssetId {
  const found = tokenOptions.find((token) => !token.disabled);
  if (found) {
    return found.tokenId;
  } else if (tokenOptions.length === 1) {
    return tokenOptions[0].tokenId;
  }

  return tokenOptions[0].tokenId;
}
