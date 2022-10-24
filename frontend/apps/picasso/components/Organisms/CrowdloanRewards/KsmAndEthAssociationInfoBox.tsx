import { Box, Button, Grid, Typography, useTheme } from "@mui/material";
import { AlertBox, Input, Link } from "@/components";
import { ConnectedAccount } from "substrate-react";
import { useStore } from "@/stores/root";

type KsmAndEthAssociationInfoBoxProps = {
    connectedAccount?: ConnectedAccount;
    isEligibleForBothAddresses?: boolean
}

export const KsmAndEthAssociationInfoBox: React.FC<KsmAndEthAssociationInfoBoxProps> = ({ connectedAccount, isEligibleForBothAddresses = false }) => {
    const theme = useTheme();
    const { ui: { openPolkadotModal } } = useStore();
    return (
        isEligibleForBothAddresses ?
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

                    <AlertBox status="error" mt={theme.spacing(4)} link={<Link href="#" onClick={openPolkadotModal}>Change address</Link>}>
                        <Typography>
                            Connected wallet addresses have both been used to contribute to Picasso crowdloan, please switch to a different
                            Picasso account to claim rewards.
                        </Typography>
                    </AlertBox>

                    <Grid container spacing={2} mt={theme.spacing(4)}>
                        <Grid item xs={12} md={6} justifyContent="flex-start">
                            <Button disabled={true} color="primary" fullWidth variant="contained">
                                Approve Transaction
                            </Button>
                        </Grid>

                        <Grid item xs={12} md={6} justifyContent="flex-end">
                            <Button disabled={true} color="primary" fullWidth variant="contained">
                                Approve SS58 Address
                            </Button>
                        </Grid>
                    </Grid>
                </Grid>
            </Box> : null
    );
};
