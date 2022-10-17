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

type KSMClaimFormProps = {
  availableToClaim: BigNumber;
  totalRewards: BigNumber;
  claimedRewards: BigNumber;
  amountContributed: BigNumber;
  picassoAccountName: string;
  onClaim: () => Promise<any>;
  disabled?: boolean;
  readonlyAvailableToClaim: boolean;
  readonlyTotalPicaVested: boolean;
  readonlyCrowdLoanContribution: boolean;
  readonlySS8Address: boolean;
  isClaiming: boolean;
};

export const KSMClaimForm: React.FC<KSMClaimFormProps> = ({
  disabled,
  availableToClaim,
  totalRewards,
  claimedRewards,
  amountContributed,
  picassoAccountName,
  readonlyAvailableToClaim,
  readonlyTotalPicaVested,
  readonlyCrowdLoanContribution,
  onClaim,
  isClaiming
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
        <Box>
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
                tokenId="ksm"
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
        </Box>
        <Box sx={{ mt: theme.spacing(9) }}>
          <Input
            icon="/networks/polkadot_js_wallet.svg"
            noBorder={true}
            value={picassoAccountName}
            disabled={true}
            fullWidth
            LabelProps={{
              mainLabelProps: {
                label: "Approved account",
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
            onClick={onClaim}
            variant="contained"
            color="primary"
            disabled={disabled ? disabled : false}
            fullWidth
          >
            <Typography variant="button">Claim rewards</Typography>
          </Button>
        </Box>
      </Paper>
      <Modal
        open={isClaiming}
        maxWidth="md"
        dismissible
      >
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
