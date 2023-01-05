import * as React from "react";
import { FC } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableContainerProps,
  TableHead,
  TableRow,
  Typography
} from "@mui/material";
import { BaseAsset, TokenAsset, TokenPairAsset } from "../Atom";
import { NoAssetsCover } from "./NoAssetsCover";
import { getTokenString } from "@/components/Organisms/Bond/utils";
import { humanBalance } from "shared";
import { useClaim } from "@/stores/defi/polkadot/bonds/useClaim";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";

export type MyBondsTableProps = TableContainerProps & {
  onRowClick?: (offerId: string) => void;
  activeBonds: ActiveBond[];
};

export const BondTableRow: FC<{
  bond: BondOffer;
  onRowClick: (value: string) => void;
}> = ({ bond, onRowClick }) => {
  const { claimable, pending, vestingTime } = useClaim(bond.bondOfferId);
  return (
    <TableRow
      sx={{
        "&:hover": {
          cursor: "pointer"
        }
      }}
      key={getTokenString(bond.reward.asset)}
      onClick={() => onRowClick(bond.bondOfferId)}
    >
      <TableCell align="left">
        {Array.isArray(bond.asset) && (
          <TokenPairAsset tokenIds={bond.asset.map(({ id }) => id)} />
        )}
        {!Array.isArray(bond.asset) && <TokenAsset tokenId={bond.asset.id} />}
      </TableCell>
      <TableCell align="left">
        <BaseAsset
          icon={
            Array.isArray(bond.reward.asset)
              ? bond.reward.asset[0].icon
              : bond.reward.asset.icon
          }
          label={`${humanBalance(claimable)} ${
            Array.isArray(bond.reward.asset)
              ? bond.reward.asset[0].symbol
              : bond.reward.asset.symbol
          }`}
        />
      </TableCell>
      <TableCell align="left">
        <BaseAsset
          icon={
            Array.isArray(bond.reward.asset)
              ? bond.reward.asset[0].icon
              : bond.reward.asset.icon
          }
          label={`${humanBalance(pending)} ${
            Array.isArray(bond.reward.asset)
              ? bond.reward.asset[0].symbol
              : bond.reward.asset.symbol
          }`}
        />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{vestingTime}</Typography>
      </TableCell>
    </TableRow>
  );
};

export const MyBondsTable: React.FC<MyBondsTableProps> = ({
  activeBonds,
  onRowClick = () => {
  },
  ...rest
}) => {
  if (activeBonds.length > 0) {
    return (
      <TableContainer {...rest}>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Asset</TableCell>
              <TableCell align="left">Claimable</TableCell>
              <TableCell align="left">Pending</TableCell>
              <TableCell align="left">Vesting Time</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {activeBonds
              .filter((bond) => bond.alreadyClaimed === 0)
              .map(({ bond }: { bond: BondOffer }, index) => (
                <BondTableRow key={index} bond={bond} onRowClick={onRowClick} />
              ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  } else {
    return <NoAssetsCover />;
  }
};
