import { Box, Button, Grid, Typography, useTheme } from "@mui/material";
import { AlertBox, Input, Link } from "@/components";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { useStore } from "@/stores/root";

type KsmAndEthAssociationInfoBoxProps = {
  connectedAccount?: InjectedAccountWithMeta;
  isEligibleForBothAddresses?: boolean;
};

export const KsmAndEthAssociationInfoBox: React.FC<
  KsmAndEthAssociationInfoBoxProps
> = ({ connectedAccount, isEligibleForBothAddresses = false }) => {
  const theme = useTheme();
  const {
    ui: { openPolkadotModal },
  } = useStore();
  return isEligibleForBothAddresses ? (
    <Box>
      <Grid item xs={12} md={12}>
        <Input
          icon="/networks/polkadot_js_wallet.svg"
          noBorder={true}
          value={connectedAccount?.address}
          disabled={true}
          fullWidth
          LabelProps={{
            mainLabelProps: {
              label: "SS58 Address",
              TooltipProps: {
                title:
                  "Please connect an account address that contributed to the crowd loan.",
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

        <AlertBox
          status="error"
          mt={theme.spacing(4)}
          link={
            <Link href="#" onClick={openPolkadotModal}>
              Change address
            </Link>
          }
        >
          <Typography>
            If you have used both an Ethereum and Kusama wallet to contribute to
            the Picasso crowd loan, please use separate or derivative Picasso
            wallets to claim your rewards.
          </Typography>
        </AlertBox>

        <Grid container spacing={2} mt={theme.spacing(4)}>
          <Grid item xs={12} md={6} justifyContent="flex-start">
            <Button
              disabled={true}
              color="primary"
              fullWidth
              variant="contained"
            >
              Approve Transaction
            </Button>
          </Grid>

          <Grid item xs={12} md={6} justifyContent="flex-end">
            <Button
              disabled={true}
              color="primary"
              fullWidth
              variant="contained"
            >
              Approve SS58 Address
            </Button>
          </Grid>
        </Grid>
      </Grid>
    </Box>
  ) : null;
};
