import { useTheme } from "@mui/material";
import { useRouter } from "next/router";
import { FeaturedBox } from "../Molecules/FeaturedBox";

export const ConnectWalletFeaturedBox: React.FC<{ message?: string }> = ({
  message,
}) => {
  const router = useRouter();
  const theme = useTheme();

  return (
    <FeaturedBox
      title="Connect wallet"
      textBelow={
        message ?? "To see your portfolio, wallet needs to be connected."
      }
      sx={{
        padding: theme.spacing(6),
      }}
    />
  );
};
