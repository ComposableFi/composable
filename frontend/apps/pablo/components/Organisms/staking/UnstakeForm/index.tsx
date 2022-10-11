import { Box, Button, Grid, Typography } from "@mui/material";
import { Alert } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import { CheckableXPabloItemBox } from "./CheckableXPabloItemBox";
import { useState } from "react";
import { UnstakeModal } from "./UnstakeModal";
import { PBLO_ASSET_ID, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useXTokensList } from "@/defi/hooks";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { useUnstake } from "@/defi/hooks/stakingRewards";
import { useStakingRewardPoolCollectionId } from "@/store/stakingRewards/stakingRewards.slice";
import { StakingRewardPool } from "@/defi/types";
import BigNumber from "bignumber.js";

export const UnstakeForm: React.FC<BoxProps & {
  stakingRewardPool?: StakingRewardPool;
}> = ({ stakingRewardPool, ...boxProps }) => {
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const financialNftCollectionId = useStakingRewardPoolCollectionId(
    PBLO_ASSET_ID
  );

  const xPablos = useXTokensList({
    stakedAssetId: PBLO_ASSET_ID,
  });
  const [selectedXPabloId, setSelectedXPabloId] = useState<
    string | undefined
  >();

  const hasStakedPositions = xPablos.length > 0;
  const selectedXPablo =
    selectedXPabloId && xPablos.find((item) => item.nftId == selectedXPabloId);

  const handleUnstake = useUnstake({
    financialNftCollectionId: financialNftCollectionId
      ? BigNumber(financialNftCollectionId)
      : undefined,
    financialNftInstanceId: selectedXPablo
      ? BigNumber(selectedXPablo.nftId)
      : undefined,
  });

  const expired = selectedXPablo && selectedXPablo.isExpired;

  const [isUnstakeModalOpen, setIsUnstakeModalOpen] = useState<boolean>(false);

  const onSelectUnstake = () => {
    setIsUnstakeModalOpen(true);
  };

  const inUnstaking = usePendingExtrinsic(
    "unstake",
    "stakingRewards",
    selectedAccount ? selectedAccount.address : "-"
  );

  return (
    <Box {...boxProps}>
      <Box display="flex" flexDirection="column" gap={3}>
        {!hasStakedPositions && (
          <Box>
            <Typography
              variant="subtitle1"
              color="text.primary"
              textAlign={"center"}
            >
              No PBLOs Staked.
            </Typography>
            <Typography
              mt={2}
              variant="body2"
              color="text.secondary"
              textAlign={"center"}
            >
              You currently do not have any active PBLO staked positions.
            </Typography>
          </Box>
        )}

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
              onClick={onSelectUnstake}
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
          stakingRewardPool={stakingRewardPool}
          xPablo={selectedXPablo}
          open={isUnstakeModalOpen}
          onDismiss={() => {
            setIsUnstakeModalOpen(false);
          }}
          onUnstake={() => {
            setIsUnstakeModalOpen(false);
            handleUnstake();
          }}
        />
      )}

      <ConfirmingModal open={inUnstaking} />
    </Box>
  );
};
