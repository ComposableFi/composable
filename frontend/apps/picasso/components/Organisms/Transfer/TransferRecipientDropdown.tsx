import { RecipientDropdown } from "@/components";
import React, { useEffect } from "react";
import { useStore } from "@/stores/root";
import { useKusamaAccounts, usePicassoAccounts } from "@/defi/polkadot/hooks";
import { resetRecipient } from "@/stores/defi/polkadot/transfers/subscribers/resetRecipient";

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
function prependEmpty(
  items: Array<{
    disabled?: boolean;
    value: string;
    label: string;
    icon: string;
  }>
): Array<{ value: string; label: string; icon: string; disabled?: boolean }> {
  return [
    {
      value: "",
      label: "Please select",
      disabled: true,
      icon: "",
    },
    ...items,
  ];
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
  const recipients = useStore((state) => state.transfers.recipients);
  const to = useStore((state) => state.transfers.networks.to);
  const picassoAccounts = usePicassoAccounts();
  const kusamaAccounts = useKusamaAccounts();
  const updateRecipient = useStore((state) => state.transfers.updateRecipient);
  const options =
    to === "kusama"
      ? prependEmpty(
          composeOptions(attachNetworkIconToItems("kusama")(kusamaAccounts))
        )
      : prependEmpty(
          composeOptions(attachNetworkIconToItems("picasso")(picassoAccounts))
        );
  const handleRecipientChange = (value: string) => updateRecipient(value);

  useEffect(() => {
    const unsub = resetRecipient();

    return () => unsub();
  }, []);

  return (
    <RecipientDropdown
      value={recipients.selected}
      expanded={false}
      options={options}
      setValue={handleRecipientChange}
    />
  );
};
