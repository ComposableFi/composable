import { setUiState } from "@/store/ui/ui.slice";
import { FeaturedBox, FeaturedBoxProps } from "../Molecules/FeaturedBox";

export const ConnectWalletFeaturedBox: React.FC<FeaturedBoxProps> = ({
  ButtonProps,
  ...featuredBoxProps
}) => {
  return (
    <FeaturedBox
      title="Connect wallet"
      textBelow="Your current pool positions will appear here."
      TextBelowProps={{fontWeight: 400}}
      p={6}
      ButtonProps={{
        label: "Connect Wallet",
        onClick: () => {
          setUiState({ isPolkadotModalOpen: true });
        },
        ...ButtonProps
      }}
      {...featuredBoxProps}
    />
  );
};
