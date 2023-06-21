import {
  Box,
  Paper,
  Skeleton,
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

export const StakingPortfolioLoadingState = () => {
  const theme = useTheme();
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
                <TableCell>Your xPICA</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              <TableRow>
                <TableCell>
                  <Skeleton variant="circular" width={48} height={48} />
                </TableCell>
                <TableCell>
                  <Skeleton variant="rectangular" height={48}></Skeleton>
                </TableCell>
                <TableCell>
                  <Skeleton variant="rectangular" height={48}></Skeleton>
                </TableCell>
                <TableCell><Skeleton variant="rectangular" height={48}></Skeleton></TableCell>
                <TableCell><Skeleton variant="rectangular" height={48}></Skeleton></TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Skeleton variant="circular" width={48} height={48} />
                </TableCell>
                <TableCell>
                  <Skeleton variant="rectangular" height={48}></Skeleton>
                </TableCell>
                <TableCell>
                  <Skeleton variant="rectangular" height={48}></Skeleton>
                </TableCell>
                <TableCell><Skeleton variant="rectangular" height={48}></Skeleton></TableCell>
                <TableCell><Skeleton variant="rectangular" height={48}></Skeleton></TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </TableContainer>
      </Stack>
    </Paper>
  );
};
