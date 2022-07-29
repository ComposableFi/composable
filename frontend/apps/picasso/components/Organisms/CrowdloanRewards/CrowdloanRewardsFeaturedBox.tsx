import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { useTheme } from "@mui/material";
import { useRouter } from "next/router";
import { useContext } from "react";
import { FeaturedBox } from "../../Molecules/FeaturedBox";

export const CrowdloanRewardsFeaturedBox: React.FC<{}> = () => {
  const theme = useTheme();
  const router = useRouter();
  const { extensionStatus } = useContext(ParachainContext);

  return (
    <FeaturedBox
      title="Crowdloan Rewards"
      textBelow="Claim your PICA rewards for both KSM and stablecoin contributions."
      horizontalAligned
      ButtonProps={{
        label: "Claim rewards",
        onClick: () => {
          router.push("/crowdloan-rewards");
        },
        variant: "contained",
        disabled: extensionStatus !== "connected",
      }}
      sx={{
        padding: theme.spacing(6),
      }}
    />
  );
};
