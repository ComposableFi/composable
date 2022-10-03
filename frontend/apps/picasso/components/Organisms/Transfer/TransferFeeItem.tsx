import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Typography,
} from "@mui/material";
import React, { FC, useMemo } from "react";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { useStore } from "@/stores/root";
import { Select } from "@/components";
import { AssetId } from "@/defi/polkadot/types";

const headerStyle = {
  "&.MuiAccordionSummary-root": {
    paddingRight: 0,
  },

  "&.Mui-expanded": {
    minHeight: "3rem",

    "& > .MuiAccordionSummary-content.Mui-expanded": {
      margin: 0,
    },
  },
};

const expandIconStyle = {
  color: "primary.light",
};

const detailsStyle = {
  "&.MuiAccordionDetails-root": {
    marginTop: "1rem",
    marginBottom: "0.5rem",
  },
};
export const TransferFeeItem: FC<{}> = () => {
  const { hasFeeItem, feeItem, setFeeItem, toggleHasFee } = useStore(
    ({ transfers }) => transfers
  );
  const balances = useStore(
    ({ substrateBalances }) => substrateBalances.assets
  );
  const from = useStore(({ transfers }) => transfers.networks.from);
  const options = useMemo(() => {
    const { assets } = balances[from];
    const items = Object.entries(assets)
      .filter(([_, asset]) => !asset.balance.isZero())
      .map(([symbol, asset]) => ({
        value: symbol,
        label: asset.meta.name,
        icon: asset.meta.icon,
        disabled: asset.balance.isZero(),
      }));

    return [
      { value: "", label: "Please select an item", disabled: true },
      ...items,
    ];
  }, [balances, from]);

  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    setFeeItem(item.target.value as AssetId);
  };

  if (options.length === 1) {
    return null;
  }

  return (
    <Accordion expanded={hasFeeItem} onChange={toggleHasFee}>
      <AccordionSummary
        sx={headerStyle}
        expandIcon={<ExpandMoreIcon sx={expandIconStyle} />}
        aria-controls="recipient-content"
        id="recipient-header"
      >
        <Typography variant="body2" color="primary.light">
          Pay fee in different asset
        </Typography>
      </AccordionSummary>
      <AccordionDetails sx={detailsStyle}>
        <Select value={feeItem} options={options} onChange={handleChangeItem} />
      </AccordionDetails>
    </Accordion>
  );
};
