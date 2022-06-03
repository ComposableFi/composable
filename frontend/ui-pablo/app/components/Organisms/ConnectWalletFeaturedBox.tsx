import useStore from "@/store/useStore";
import { FeaturedBox, FeaturedBoxProps } from "../Molecules/FeaturedBox";

export const ConnectWalletFeaturedBox: React.FC<FeaturedBoxProps> = ({
  ButtonProps,
  ...featuredBoxProps
}) => {
  const { openPolkadotModal } = useStore();

  return (
    <FeaturedBox
      title="Connect wallet"
      textBelow="Your current pool positions will appear here."
      TextBelowProps={{fontWeight: 400}}
      p={6}
      ButtonProps={{
        label: "Connect Wallet",
        onClick: () => {
          openPolkadotModal();
        },
        ...ButtonProps
      }}
      {...featuredBoxProps}
    />
  );
};
