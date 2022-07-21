import { PairAsset, BaseAsset } from "@/components/Atoms";
import { useBlockInterval } from "@/defi/hooks";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondOffer } from "@/defi/types";
import { TableCell, TableRow, Typography } from "@mui/material";
import { useMemo } from "react";
import moment from "moment";

const BondedOfferRow = ({ bondOffer }: { bondOffer: BondOffer }) => {
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;
  const blockTime = useBlockInterval();

  const vestingTime = useMemo(() => {
    if (bondOffer.maturity === "Infinite") {
      return "Infinite";
    }

    if (blockTime) {
      const duration = bondOffer.maturity.Finite.returnIn.times(blockTime.toString()).toNumber();
      const inDuration = moment.duration(duration);
      return moment.utc(inDuration.as('milliseconds')).format('HH:mm:ss')
    }

    return "00:00:00";
  }, [blockTime, bondOffer])

  return (
    <TableRow sx={{ cursor: "pointer" }}>
      <TableCell align="left">
        {baseAsset && quoteAsset ? (
          <PairAsset
            assets={[
              {
                icon: baseAsset.icon,
                label: baseAsset.symbol,
              },
              {
                icon: quoteAsset.icon,
                label: quoteAsset.symbol,
              },
            ]}
            separator="/"
          />
        ) : simplePrincipalAsset ? (
          <BaseAsset label={simplePrincipalAsset.symbol} icon={simplePrincipalAsset.icon} />
        ) : null }
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {0} CHAOS
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {0} CHAOS
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{vestingTime}</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondedOfferRow;