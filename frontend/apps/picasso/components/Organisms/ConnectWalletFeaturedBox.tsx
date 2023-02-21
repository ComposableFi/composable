import { track } from "@/utils/Analytics";
import { useCurrentPage } from "@/utils/Analytics/useCurrentPage";
import { useTheme } from "@mui/material";
import { FeaturedBox } from "../Molecules/FeaturedBox";

export const ConnectWalletFeaturedBox = ({ message }: { message?: string }) => {
  const theme = useTheme();
  const { label } = useCurrentPage();

  return (
    <FeaturedBox
      title="Connect wallet"
      textBelow={
        message ?? "To see your portfolio, wallet needs to be connected."
      }
      sx={{
        padding: theme.spacing(6),
      }}
      ButtonProps={{
        label: "Connect wallet",
        onClick: () => {
          track({
            category: label ?? "-",
            action: "Clicked connect wallet CTA",
            label: "connect wallet",
          });
          document.dispatchEvent(new CustomEvent("WalletConnect"));
        },
      }}
    />
  );
};
