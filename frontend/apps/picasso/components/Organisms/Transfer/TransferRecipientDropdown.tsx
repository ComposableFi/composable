import { RecipientDropdown } from "@/components";
import React from "react";
import { useStore } from "@/stores/root";
import { useKusamaProvider, usePicassoProvider } from "@/defi/polkadot/hooks";

function attachNetworkIconToItems(network: "kusama" | "picasso") {
  return (items: any[]) => {
    return items.map((item: any) => {
      return {
        ...item,
        icon:
          network === "kusama"
            ? "/networks/kusama.svg"
            : "/networks/picasso.svg",
      };
    });
  };
}

function composeOptions(
  items: Array<{ address: string; name: string; icon: string }>
): Array<{ value: string; label: string; icon: string }> {
  return items.map((item: any) => {
    return {
      value: item.address,
      label: item.name,
      icon: item.icon,
    };
  });
}

export const TransferRecipientDropdown = () => {
  const {
    recipients,
    updateRecipient,
    networks: { to: toNetwork },
  } = useStore(({ transfers }) => transfers);
  const { accounts: picassoAccounts } = usePicassoProvider();
  const { accounts: kusamaAccounts } = useKusamaProvider();
  const options =
    toNetwork === "kusama"
      ? composeOptions(attachNetworkIconToItems("kusama")(kusamaAccounts))
      : composeOptions(attachNetworkIconToItems("picasso")(picassoAccounts));
  const handleRecipientChange = (value: string) => updateRecipient(value);

  return (
    <RecipientDropdown
      value={recipients.selected}
      expanded={false}
      options={options}
      setValue={handleRecipientChange}
    />
  );
};
