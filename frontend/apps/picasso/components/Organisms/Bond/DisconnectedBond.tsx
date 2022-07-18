import { Grid } from "@mui/material";
import { ConnectWalletFeaturedBox } from "@/components";
import { ConnectToStakeCover } from "@/components/Molecules/ConnectToStakeCover";

const standardPageSize = {
  xs: 12,
};

export const DisconnectedBond = () => (
  <>
    <Grid item {...standardPageSize}>
      <ConnectWalletFeaturedBox message="To start staking, wallet needs to be connected."/>
    </Grid>
    <Grid item {...standardPageSize}>
      <ConnectToStakeCover message="Connect to check your active positions."/>
    </Grid>
  </>
);
