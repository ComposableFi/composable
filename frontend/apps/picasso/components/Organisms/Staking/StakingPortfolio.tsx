import { FC } from "react";
import {
  Box,
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
import { dateFromNumber, formatDate } from "shared";
import { useStore } from "@/stores/root";

export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  const openPositions = useStore(({ staking }) => staking.openPositions);
  if (!openPositions) {
    return <h1>Portfolio empty</h1>;
  }

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        <TableContainer component={Box}>
          <Table>
            <TableHead>
              <TableRow>
                <TableCell>fNFTID</TableCell>
                <TableCell>Locked Pica</TableCell>
                <TableCell>Expiry Date</TableCell>
                <TableCell>Multiplier</TableCell>
                <TableCell>Your CHAOS</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {openPositions.map((item, key) => (
                <TableRow key={key}>
                  <TableCell>
                    <TokenAsset tokenId={"pica"} label={item.id} />
                  </TableCell>
                  <TableCell>{item.lockedPica.toFixed()}</TableCell>
                  <TableCell>
                    {formatDate(dateFromNumber(item.expiryDate))}
                  </TableCell>
                  <TableCell>{item.multiplier.toFixed(1)}</TableCell>
                  <TableCell>{item.yourChaos.toFixed()}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Stack>
    </Paper>
  );
};
