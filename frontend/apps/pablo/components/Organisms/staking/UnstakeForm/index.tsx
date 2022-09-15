import { Box, Button, Grid } from "@mui/material";
import { Alert } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import { CheckableXPabloItemBox } from "./CheckableXPabloItemBox";
import { useState } from "react";
import { UnstakeModal } from "./UnstakeModal";
import { useStakingPositions } from "@/store/hooks/useStakingPositions";
import { PBLO_ASSET_ID } from "@/defi/utils";

export const UnstakeForm: React.FC<BoxProps> = ({ ...boxProps }) => {
  const xPablos = useStakingPositions({
    stakedAssetId: PBLO_ASSET_ID
  })
  const [selectedXPabloId, setSelectedXPabloId] = useState<
    string | undefined
  >();

  const selectedXPablo =
    selectedXPabloId &&
    xPablos.find((item) => item.nftId == selectedXPabloId);

  const expired =
    selectedXPablo && selectedXPablo.isExpired;

  const [isUnstakeModalOpen, setIsUnstakeModalOpen] = useState<boolean>(false);

  const handleUnstake = () => {
    setIsUnstakeModalOpen(true);
  };

  return (
    <Box {...boxProps}>
      <Box display="flex" flexDirection="column" gap={3}>
        {xPablos.map((xPablo) => (
          <CheckableXPabloItemBox
            key={xPablo.nftId}
            xPablo={xPablo}
            selectedXPabloId={selectedXPabloId}
            setSelectedXPabloId={setSelectedXPabloId}
          />
        ))}
      </Box>
      {expired && (
        <Box mt={3}>
          <Alert
            severity="warning"
            alertTitle="Slash warning"
            alertText="If you withdraw now you will get rekt with less PICA."
            AlertTextProps={{ color: "text.secondary" }}
          />
        </Box>
      )}

      <Box mt={3}>
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <Button
              onClick={handleUnstake}
              fullWidth
              variant="contained"
              disabled={!selectedXPablo}
            >
              Burn and unstake
            </Button>
          </Grid>
        </Grid>
      </Box>

      {selectedXPablo && (
        <UnstakeModal
          dismissible
          xPablo={selectedXPablo}
          open={isUnstakeModalOpen}
          onClose={() => setIsUnstakeModalOpen(false)}
        />
      )}
    </Box>
  );
};
