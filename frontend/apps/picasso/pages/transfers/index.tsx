import React from "react";
import { NextPage } from "next";
import { Box, Button, Grid, Theme, Typography } from "@mui/material";
import { SwapHoriz } from "@mui/icons-material";

import { useStore } from "@/stores/root";
import Default from "@/components/Templates/Default";
import {
  FeeDisplay,
  NetworkSelect,
  RecipientDropdown,
  TextSwitch,
  TokenDropdownCombinedInput
} from "@/components";
import { PageTitle } from "@/components";
import { TokenId } from "tokens";
import { formatToken } from "shared";

const gridContainerStyle = {
  mx: "auto"
};

const gridItemStyle = (pt: string = "2rem") => ({
  xs: 12,
  sx: { pt }
});

const networksStyle = (theme: Theme) =>
  ({
    alignItems: "flex-end",
    flexDirection: "row",
    gap: "2rem",
    [theme.breakpoints.down("sm")]: {
      flexDirection: "column",
      alignItems: "initial",
      gap: "1.5rem"
    },
    "& > *": { flex: 1 }
  } as const);

const swapButtonStyle = (theme: Theme) => ({
  maxWidth: "4rem",
  minWidth: "4rem",
  [theme.breakpoints.down("sm")]: {
    maxWidth: "3.5rem",
    minWidth: "3.5rem",
    alignSelf: "center"
  }
});

const amountInputStyle = {
  "& .MuiOutlinedInput-input": {
    textAlign: "center"
  }
};

const Transfers: NextPage = () => {
  const {
    networks,
    amount,
    recipients,
    keepAlive,
    fee,
    flipKeepAlive,
    updateAmount,
    updateNetworks,
    updateRecipient
  } = useStore(({ transfers }) => transfers);

  const handleSwapClick = () =>
    updateNetworks({ from: networks.to, to: networks.from });

  const handleUpdateFromValue = (value: string) =>
    updateNetworks({ ...networks, from: value });

  const handleUpdateToValue = (value: string) =>
    updateNetworks({ ...networks, to: value });

  const handleAmountChange = (event: React.ChangeEvent<HTMLInputElement>) =>
    updateAmount({ ...amount, value: +event.target.value });

  const handleTokenChange = (event: React.ChangeEvent<HTMLInputElement>) =>
    updateAmount({ ...amount, tokenId: event.target.value as TokenId });

  const handleMaxClick = () =>
    updateAmount({ ...amount, value: amount.balance });

  const handleRecipientChange = (value: string) => updateRecipient(value);

  const handleKeepAliveChange = (_: React.ChangeEvent<HTMLInputElement>) =>
    flipKeepAlive();

  return (
    <Default>
      <Grid
        container
        sx={gridContainerStyle}
        maxWidth={1032}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...gridItemStyle("6rem")}>
          <PageTitle
            title="Transfers"
            subtitle="You will be able to move assets on any available Kusama chains."
            textAlign="center"
          />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <Box display="flex" sx={networksStyle}>
            <NetworkSelect
              LabelProps={{ mainLabelProps: { label: "From network" } }}
              options={networks.options}
              value={networks.from}
              searchable
              substrateNetwork
              setValue={handleUpdateFromValue}
            />
            <Button
              sx={swapButtonStyle}
              variant="outlined"
              size="large"
              onClick={handleSwapClick}
            >
              <SwapHoriz />
            </Button>
            <NetworkSelect
              LabelProps={{ mainLabelProps: { label: "To network" } }}
              options={networks.options}
              value={networks.to}
              searchable
              substrateNetwork
              setValue={handleUpdateToValue}
            />
          </Box>
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TokenDropdownCombinedInput
            buttonLabel="Max"
            value={amount.value}
            LabelProps={{
              mainLabelProps: {
                label: "Amount"
              },
              balanceLabelProps: {
                label: "Balance:",
                balanceText: formatToken(amount.balance, amount.tokenId)
              }
            }}
            ButtonProps={{
              onClick: handleMaxClick
            }}
            InputProps={{
              sx: amountInputStyle
            }}
            CombinedSelectProps={{
              value: amount.tokenId,
              options: amount.options,
              onChange: handleTokenChange
            }}
            onChange={handleAmountChange}
          />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <RecipientDropdown
            value={recipients.selected}
            expanded={false}
            options={recipients.options}
            setValue={handleRecipientChange}
          />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <FeeDisplay
            label="Fee"
            feeText={formatToken(fee, amount.tokenId)}
            TooltipProps={{
              title: "Fee tooltip title"
            }}
          />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TextSwitch
            label="Keep alive"
            checked={keepAlive}
            TooltipProps={{
              title:
                "This will prevent account of being removed due to low balance."
            }}
            onChange={handleKeepAliveChange}
          />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <Button
            variant="contained"
            color="primary"
            disabled={amount.value <= 0 || amount.value > amount.balance}
            fullWidth
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
        </Grid>
      </Grid>
    </Default>
  );
};

export default Transfers;
