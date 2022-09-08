import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { Box, Grid, useTheme } from "@mui/material";
import { StakingDisconnected } from "@/components/Organisms/Staking/StakingDisconnected";
import { useContext } from "react";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { ConnectorType, useConnector } from "bi-lib";
import { PageTitle } from "@/components";
import { StakingConnected } from "@/components/Organisms/Staking/StakingConnected";

const Staking: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useContext(ParachainContext);
  const { isActive } = useConnector(ConnectorType.MetaMask);
  const isDisconnected = !isActive || extensionStatus !== "connected";
  const standardPageSize = {
    xs: 12,
  };

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
        <Grid container>
          <Grid item {...standardPageSize}>
            <PageTitle
              title="Staking"
              subtitle="Lock PICA to mint CHAOS, the yield bearing governance fNFT."
              textAlign="center"
            />
          </Grid>
        </Grid>
        {isDisconnected && (
          <StakingDisconnected gridSize={standardPageSize} theme={theme} />
        )}
        {!isDisconnected && <StakingConnected />}
      </Box>
    </Default>
  );
};

export default Staking;
