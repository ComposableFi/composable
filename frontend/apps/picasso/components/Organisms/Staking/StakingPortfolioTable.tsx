import {
  PortfolioItem,
  StakingPortfolio as TStakingPortfolio,
} from "@/stores/defi/polkadot/stakingRewards/slice";
import {
  Box,
  Button,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  useTheme,
} from "@mui/material";
import { TokenAsset } from "@/components";
import { humanBalance } from "shared";
import { Add } from "@mui/icons-material";
import { useState } from "react";
import { RenewModal } from "@/components/Organisms/Staking/RenewModal";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";

export const PortfolioRow = ({
  portfolio,
  onSelectToken,
}: {
  portfolio: PortfolioItem;
  onSelectToken: (collectionId: string, instanceId: string) => void;
}) => {
  const theme = useTheme();
  const { isExpired, portfolioDate } = useExpiredPortfolio(portfolio);
  console.log(JSON.stringify(portfolio, null, 2));
  return (
    <TableRow>
      <TableCell>
        <TokenAsset tokenId={"pica"} label={`fNFT ${portfolio.instanceId}`} />
      </TableCell>
      <TableCell>{humanBalance(portfolio.stake)}</TableCell>
      <TableCell>{portfolioDate}</TableCell>
      <TableCell>{`${portfolio.multiplier.toFixed(2)}%`}</TableCell>
      <TableCell>≈{portfolio.share.toFixed(2)}</TableCell>
      <TableCell>
        {!isExpired && (
          <Button
            variant="outlined"
            size="small"
            sx={{
              minWidth: theme.spacing(6),
            }}
            onClick={() => {
              onSelectToken(portfolio.collectionId, portfolio.instanceId);
            }}
          >
            <Add />
          </Button>
        )}
      </TableCell>
    </TableRow>
  );
};

export const StakingPortfolioTable = ({
  stakingPortfolio,
}: {
  stakingPortfolio: TStakingPortfolio;
}) => {
  const [selectedToken, setSelectedToken] = useState<[string, string]>([
    "",
    "",
  ]);
  const [isRenewModalOpen, setIsRenewModalOpen] = useState<boolean>(false);
  return (
    <>
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
            {stakingPortfolio.map((portfolio, key) => (
              <PortfolioRow
                key={key}
                portfolio={portfolio}
                onSelectToken={(collectionId, instanceId) => {
                  setSelectedToken([collectionId, instanceId]);
                  setIsRenewModalOpen(true);
                }}
              />
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <RenewModal
        open={isRenewModalOpen}
        selectedToken={selectedToken}
        onClose={() => setIsRenewModalOpen(false)}
      />
    </>
  );
};
