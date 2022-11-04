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
