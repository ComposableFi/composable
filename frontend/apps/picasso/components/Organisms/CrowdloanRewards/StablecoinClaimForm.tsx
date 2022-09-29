import { BigNumberInput, Input, Modal } from "@/components";
import {
  Box,
  Button,
  CircularProgress,
  Grid,
  Paper,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import React from "react";

type StablecoinClaimFormProps = {
  availableToClaim: BigNumber;
  totalRewards: BigNumber;
  claimedRewards: BigNumber;
  amountContributed: BigNumber;
  picassoAccountName: string;
  SS58Address: string;
  onClaim: () => void;
  needsApproval?: boolean;
  readonlyAvailableToClaim: boolean;
  readonlyTotalPicaVested: boolean;
  readonlyCrowdLoanContribution: boolean;
  readonlySS8Address: boolean;
  isClaiming: boolean;
  disabled: boolean | undefined;
};

export const StablecoinClaimForm: React.FC<StablecoinClaimFormProps> = ({
  disabled,
  availableToClaim,
  totalRewards,
  claimedRewards,
  amountContributed,
  SS58Address,
  readonlyAvailableToClaim,
  readonlyTotalPicaVested,
  readonlyCrowdLoanContribution,
  onClaim,
  isClaiming,
}) => {
  const theme = useTheme();

  return (
    <Box>
      <Paper
        elevation={0}
        sx={{
          padding: theme.spacing(4),
        }}
      >
        <Grid container spacing={4}>
          <Grid item xs={12} md={6}>
            <BigNumberInput
              noBorder={true}
              value={availableToClaim}
              setter={(v: BigNumber) => {}}
              tokenId="pica"
              tokenDescription={false}
              isValid={(_v: boolean) => {}}
              placeholder="0"
              maxDecimals={18}
              maxValue={new BigNumber(1e18)}
              disabled={readonlyAvailableToClaim}
              LabelProps={{
                mainLabelProps: {
                  label: "Available to claim",
                  TooltipProps: {
                    title:
                      "This is your total PICA available to claim now for your account",
                    children: <></>,
                  },
                },
              }}
              InputProps={{
                inputProps: {
                  sx: {
                    textAlign: "center",
                  },
                },
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <BigNumberInput
              noBorder={true}
              value={claimedRewards}
              setter={(v: BigNumber) => {}}
              isValid={(_v: boolean) => {}}
              tokenId="pica"
              tokenDescription={false}
              placeholder="0"
              maxDecimals={18}
              maxValue={new BigNumber(1e18)}
              disabled={readonlyTotalPicaVested}
              LabelProps={{
                mainLabelProps: {
                  label: "Claimed",
                  TypographyProps: {
                    fontSize: 16,
                  },
                  TooltipProps: {
                    title:
                      "This is the total PICA you have claimed so far for your account",
                    children: <></>,
                  },
                },
              }}
              InputProps={{
                inputProps: {
                  sx: {
                    textAlign: "center",
                  },
                },
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <BigNumberInput
              noBorder={true}
              value={totalRewards}
              setter={(v: BigNumber) => {}}
              isValid={(_v: boolean) => {}}
              tokenId="pica"
              tokenDescription={false}
              placeholder="0"
              maxDecimals={18}
              maxValue={new BigNumber(1e18)}
              disabled={readonlyTotalPicaVested}
              LabelProps={{
                mainLabelProps: {
                  label: "Total PICA rewards (unvested)",
                  TypographyProps: {
                    fontSize: 16,
                  },
                  TooltipProps: {
                    title:
                      "This is the total PICA rewards based on your crowdloan contribution for this account. Your unvested amount is your claimable plus your claimed.",
                    children: <></>,
                  },
                },
              }}
              InputProps={{
                inputProps: {
                  sx: {
                    textAlign: "center",
                  },
                },
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <BigNumberInput
              noBorder={true}
              value={amountContributed}
              setter={(v: BigNumber) => {}}
              isValid={(_v: boolean) => {}}
              tokenId="usdc"
              tokenDescription={false}
              placeholder="0"
              maxDecimals={18}
              maxValue={new BigNumber(1e18)}
              disabled={readonlyCrowdLoanContribution}
              LabelProps={{
                mainLabelProps: {
                  label: "Crowdloan contribution",
                  TooltipProps: {
                    title:
                      "Amount of KSM you have contributed to the Picasso crowdloan on this account",
                    children: <></>,
                  },
                },
              }}
              InputProps={{
                inputProps: {
                  sx: {
                    textAlign: "center",
                  },
                },
              }}
            />
          </Grid>
        </Grid>

        <Box sx={{ mt: theme.spacing(9) }}>
          <Input
            value={SS58Address}
            disabled={true}
            onChange={(e) => {}}
            fullWidth
            LabelProps={{
              mainLabelProps: {
                label: "Approved SS8 address",
                TooltipProps: {
                  title: "Account used to contribute to crowd loan",
                  children: <></>,
                },
              },
            }}
            InputProps={{
              inputProps: {
                sx: {
                  textAlign: "center",
                  color: theme.palette.text.primary,
                },
              },
            }}
          />
        </Box>
        <Box
          sx={{
            mt: theme.spacing(4),
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            gap: theme.spacing(2),
          }}
        >
          <Button
            disabled={disabled ? disabled : false}
            onClick={onClaim}
            variant="contained"
            color="primary"
            fullWidth
          >
            <Typography variant="button">Claim</Typography>
          </Button>
        </Box>
      </Paper>
      <Modal open={isClaiming} maxWidth="md" dismissible>
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            flexDirection: "column",
            gap: theme.spacing(1),
          }}
        >
          <CircularProgress size={76} sx={{ mb: theme.spacing(8) }} />
          <Typography variant="h5">Confirming transaction</Typography>
          <Typography variant="body1" color="text.secondary">
            Confirming this transaction in your wallet.
          </Typography>
        </Box>
      </Modal>
    </Box>
  );
};
