import { TokenDropdownCombinedInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { humanBalance } from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";

export const AmountTokenDropdown = () => {
  const updateAmount = useStore((state) => state.transfers.updateAmount);
  const amount = useStore((state) => state.transfers.amount);
  const { balance, tokenId } = useExistentialDeposit();
  const assets = useStore((state) => state.substrateBalances.assets.karura);

  const handleMaxClick = () => updateAmount(balance);

  function makeTokenOptions() {
    return [...Object.values(assets.assets), assets.native].map((asset) => {
      return {
        tokenId: asset.meta.symbol,
        disabled: asset.balance.lte(0),
      };
    });
  }

  return (
    <TokenDropdownCombinedInput
      buttonLabel="Max"
      value={amount}
      LabelProps={{
        mainLabelProps: {
          label: "Amount",
        },
        balanceLabelProps: {
          label: "Balance:",
          balanceText: humanBalance(balance) + " " + tokenId.toUpperCase(),
        },
      }}
      ButtonProps={{
        onClick: handleMaxClick,
      }}
      InputProps={{
        sx: amountInputStyle,
      }}
      CombinedSelectProps={{
        options: makeTokenOptions(),
      }}
      setter={updateAmount}
    />
  );
};
