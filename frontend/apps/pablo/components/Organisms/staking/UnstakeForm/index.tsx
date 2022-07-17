import { Box, Button, Grid } from "@mui/material";
import { Alert } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import { useAppSelector } from "@/hooks/store";
import { CheckableXPabloItemBox } from "./CheckableXPabloItemBox";
import { useState } from "react";
import { UnstakeModal } from "./UnstakeModal";
import { RenewModal } from "./RenewModal";
import { XPablo } from "@/defi/types";

export const UnstakeForm: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const xPablos = useAppSelector((state) => state.polkadot.yourXPablos);
  const [selectedXPabloId, setSelectedXPabloId] = useState<number | undefined>();

  const selectedXPablo = selectedXPabloId
                          && xPablos.find((item: { id: number; }) => item.id == selectedXPabloId);

  const expired = selectedXPablo && selectedXPablo.expiry < new Date().getTime();

  const [isUnstakeModalOpen, setIsUnstakeModalOpen] = useState<boolean>(false);
  const [isRenewModalOpen, setIsRenewModalOpen] = useState<boolean>(false);

  const handleUntake = () => {
    setIsUnstakeModalOpen(true);
  };

  const handleRenew = () => {
    setIsRenewModalOpen(true);
  };

  return (
    <Box {...boxProps}>

      <Box display="flex" flexDirection="column" gap={3}>
        {xPablos.map((xPablo: XPablo) => (
          <CheckableXPabloItemBox
            key={xPablo.id}
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
            AlertTextProps={{color: "text.secondary"}}
          />
        </Box>
      )}

      <Box mt={3}>
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Button
              onClick={handleRenew}
              fullWidth
              variant="contained"
              disabled={!selectedXPablo}
            >
              Renew
            </Button>
          </Grid>
          <Grid item xs={12} md={6}>
            <Button
              onClick={handleUntake}
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
        <>
          <RenewModal
            dismissible
            xPablo={selectedXPablo}
            open={isRenewModalOpen}
            onClose={() => setIsRenewModalOpen(false)}
          />

          <UnstakeModal
            dismissible
            xPablo={selectedXPablo}
            open={isUnstakeModalOpen}
            onClose={() => setIsUnstakeModalOpen(false)}
          />
        </>
      )}
    </Box>
  );
};
