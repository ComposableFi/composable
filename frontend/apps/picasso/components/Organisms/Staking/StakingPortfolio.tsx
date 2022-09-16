import { FC } from "react";
import {
  Alert,
  Box,
  Button,
  Paper,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  useTheme
} from "@mui/material";
import { TokenAsset } from "@/components";
import { formatDate, fromChainIdUnit, humanBalance } from "shared";
import { StakingPortfolioLoadingState } from "@/components/Organisms/Staking/StakingPortfolioLoadingState";
import { Add } from "@mui/icons-material";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";


export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  const {
    stakingPortfolio,
    stakingPositions,
    isPositionsLoading
  } = useStakingRewards();

  if (!stakingPositions || isPositionsLoading) {
    return <StakingPortfolioLoadingState />;
  }

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        {Object.values(stakingPortfolio).length > 0 ? (
          <TableContainer component={Box}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>fNFT ID</TableCell>
                  <TableCell>Locked PICA</TableCell>
                  <TableCell>Expiry Date</TableCell>
                  <TableCell>Multiplier</TableCell>
                  <TableCell>Your xPICA</TableCell>
                  <TableCell></TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {stakingPositions.map((stakingPosition, key: number) => (
                  <TableRow key={key}>
                    <TableCell>
                      <TokenAsset tokenId={"pica"} label={`fNFT ${stakingPosition.fnftInstanceId}`} />
                    </TableCell>
                    <TableCell>{humanBalance(fromChainIdUnit(stakingPosition.amount))}</TableCell>
                    <TableCell>
                      {formatDate(new Date(Number(stakingPosition.endTimestamp.toString())))}
                    </TableCell>
                    <TableCell>{`${stakingPortfolio[stakingPosition.fnftCollectionId][stakingPosition.fnftInstanceId].multiplier.toFixed(
                      2)}%`}</TableCell>
                    <TableCell>{fromChainIdUnit(stakingPortfolio[stakingPosition.fnftCollectionId][stakingPosition.fnftInstanceId].share).toFixed()}</TableCell>
                    <TableCell>
                      <Button
                        variant="outlined"
                        size="small"
                        sx={{
                          minWidth: theme.spacing(6)
                        }}
                        onClick={() => console.log(
                          "add stake ",
                          stakingPosition.fnftCollectionId,
                          stakingPosition.fnftInstanceId
                        )}
                      >
                        <Add />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        ) : (
          <>
            <Alert color="info">
              No position found.
            </Alert>
          </>
        )}
      </Stack>
    </Paper>
  );
};
