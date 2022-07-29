export function availableTargetNetwork(
  network: string,
  selectedNetwork: string
) {
  switch (selectedNetwork) {
    case "kusama":
      return network === "picasso";
    case "picasso":
      return network === "kusama" || network === "karura";
    case "karura":
      return network === "picasso";
  }
}

export function getTransferToken(
  fromNetwork: string,
  toNetwork: string
): "ksm" | "kusd" {
  if (fromNetwork === "kusama") return "ksm";
  if (fromNetwork === "karura") return "kusd";
  if (fromNetwork === "picasso") return getTransferToken(toNetwork, "");

  return "ksm";
}
