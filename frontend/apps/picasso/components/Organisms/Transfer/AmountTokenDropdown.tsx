import { BigNumberInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { humanBalance } from "shared";
import { useExistentialDeposit } from "@/defi/polkadot/hooks/useExistentialDeposit";

export const AmountTokenDropdown = () => {
  const updateAmount = useStore(state => state.transfers.updateAmount);
  const amount = useStore(state => state.transfers.amount);
  const { balance, tokenId } = useExistentialDeposit();

  const handleMaxClick = () => updateAmount(balance);

  return (
    <BigNumberInput
      buttonLabel="Max"
      value={amount}
      LabelProps={{
        mainLabelProps: {
          label: "Amount"
        },
        balanceLabelProps: {
          label: "Balance:",
          balanceText: humanBalance(balance) + " " + tokenId.toUpperCase()
        }
      }}
      ButtonProps={{
        onClick: handleMaxClick
      }}
      InputProps={{
        sx: amountInputStyle
      }}
      maxValue={balance}
      setter={updateAmount}
    />
  );
};
