import { BigNumberInput, Input, Modal } from "@/components";
import { AssociationMode } from "@/stores/defi/polkadot/crowdloanRewards/slice";
import { useStore } from "@/stores/root";
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
  availableToClaim: BigNumber | number;
  totalPicaVested: BigNumber | number;
  claimedPICA: BigNumber | number;
  crowdLoanContribution: BigNumber | number;
  account: string;
  onClaim: () => Promise<any>;
  disabled?: boolean;
  readonlyAvailableToClaim: boolean;
  readonlyTotalPicaVested: boolean;
  readonlyCrowdLoanContribution: boolean;
  readonlySS8Address: boolean;
  onChange: (name: string, value: unknown) => void;
  needAssociation: boolean;
  useAssociationMode: AssociationMode;
};

export const KSMClaimForm: React.FC<KSMClaimFormProps> = ({
  disabled,
  availableToClaim,
  totalPicaVested,
  claimedPICA,
  crowdLoanContribution,
  account,
  readonlyAvailableToClaim,
  readonlyTotalPicaVested,
  readonlyCrowdLoanContribution,
  onClaim,
  onChange,
}) => {
  const theme = useTheme();
  const { isClaimingKSM, closeKSMClaimModal } = useStore(({ ui }) => ui);
  const atc =
    typeof availableToClaim === "number"
      ? new BigNumber(availableToClaim)
      : availableToClaim;
  const totalPicaVestedValue =
    typeof totalPicaVested === "number"
      ? new BigNumber(totalPicaVested)
      : totalPicaVested;
  const crowdLoanContributionValue =
    typeof crowdLoanContribution === "number"
      ? new BigNumber(crowdLoanContribution)
      : crowdLoanContribution;
  const claimedPICAValue =
    typeof claimedPICA === "number" ? new BigNumber(claimedPICA) : claimedPICA;

  const handleValueChange = (value: unknown, name: string) => {
    onChange(name, value);
  };

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
                value={atc}
                setter={(v: BigNumber) =>
                  handleValueChange(v, "availableToClaim")
                }
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
                value={claimedPICAValue}
                setter={(v: BigNumber) =>
                  handleValueChange(v, "totalPicaVested")
                }
                isValid={(_v: boolean) => {}} // TODO: Implement error state
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
                value={totalPicaVestedValue}
                setter={(v: BigNumber) =>
                  handleValueChange(v, "totalPicaVested")
                }
                isValid={(_v: boolean) => {}} // TODO: Implement error state
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
                value={crowdLoanContributionValue}
                setter={(v: BigNumber) =>
                  handleValueChange(v, "crowdLoanContribution")
                }
                isValid={(_v: boolean) => {}} // TODO: Implement error state
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
            value={account}
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
        onClose={() => closeKSMClaimModal()}
        open={isClaimingKSM}
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
