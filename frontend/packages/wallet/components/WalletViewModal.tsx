
import { Lock } from "@mui/icons-material";
import CloseIcon from "@mui/icons-material/Close";
import {
    alpha,
    Box,
    BoxProps,
    Dialog,
    DialogProps,
    Grid,
    IconButton,
    Tab,
    Tabs,
    Typography,
    useMediaQuery,
    useTheme,
} from "@mui/material";
import React from "react";
import Image from "next/image";
import BigNumber from "bignumber.js";
import { TabPanel } from "./TabPanel";
import "../styles/theme.d.ts";

export type ModalProps = DialogProps & {
    dismissible?: boolean;
    nativeIcon: string;
};

export type BadgeProps = {
    color: string,
    background: string,
    icon: JSX.Element,
    label: string
} & BoxProps

const Badge = ({ color, background, icon, label, ...props }: BadgeProps) => {
    const theme = useTheme();
    return (
        <Box sx={{
            display: "inline-flex",
            justifyContent: "center",
            alignItems: "center",
            height: "2.124rem",
            color: color,
            background: background,
            borderRadius: "12px",
            px: 1
        }} {...props}>
            {icon}
            <Typography variant="inputLabel" marginLeft={theme.spacing(1)}>{label}</Typography>
        </Box>
    )
}

export const WalletViewModal: React.FC<ModalProps> = ({
    dismissible = false,
    children,
    open,
    maxWidth,
    onClose,
    nativeIcon,
    ...props
}) => {
    const theme = useTheme();
    const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
    const balance = new BigNumber(213.40);

    return (
        <Dialog PaperProps={{
            style: {
                position: "absolute",
                top: "50px",
                bottom: 0,
                left: isMobile ? 0 : "calc(100% - 40rem)",
                right: 0,
                borderRadius: "12px",
                maxWidth: "34rem",
                maxHeight: "45.875rem"
            }
        }} open={true} {...props}>
            {dismissible && (
                <IconButton
                    sx={{
                        position: "absolute",
                        top: theme.spacing(9),
                        right: theme.spacing(9),
                        color: "primary.light",
                        borderRadius: 1,
                        "&:hover": {
                            backgroundColor: alpha(
                                theme.palette.primary.light,
                                theme.custom.opacity.light
                            ),
                            color: "secondary.main",
                        },
                    }}
                    onClick={() => onClose?.({}, "backdropClick")}
                    aria-label="close"
                >
                    <CloseIcon />
                </IconButton>
            )}
            <Grid container xs={12}>
                <Grid item xs={12} textAlign={"center"}>
                    <Box sx={{
                        display: "flex",
                        justifyContent: "center",
                        marginTop: theme.spacing(2)
                    }}>
                        <Image
                            src={nativeIcon}
                            width="24"
                            height="24"
                            alt={"icon"}
                        />
                    </Box>
                    <Box sx={{
                        display: "flex",
                        justifyContent: "center",
                        marginTop: theme.spacing(2)
                    }}>
                        <Typography
                            sx={{
                                fontSize: "2rem"
                            }}>
                            {balance.toFixed(2)}
                        </Typography>
                        <Typography
                            sx={{
                                fontSize: "2rem",
                                color: theme.palette.text.secondary,
                                marginLeft: theme.spacing(1)
                            }}>
                            PICA
                        </Typography>
                    </Box>
                    <Box sx={{
                        display: "flex",
                        justifyContent: "center",
                        marginTop: theme.spacing(2)
                    }}>
                        <Typography
                            sx={{
                                fontSize: "1rem",
                                color: theme.palette.text.secondary,
                            }}>
                            Wallet Balance
                        </Typography>
                    </Box>
                    <Box sx={{
                        display: "flex",
                        justifyContent: "center",
                        marginTop: theme.spacing(2)
                    }}>
                        <Badge icon={<Lock />} background={alpha(theme.palette.warning.main, 0.1)} color={theme.palette.warning.main} label="Locked" />
                    </Box>
                </Grid>
                <Grid item xs={12}>
                    <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                        <Tabs variant="fullWidth" value={0} onChange={(evt) => {
                            console.log('changed')
                        }} aria-label="basic tabs example">
                            <Tab label="Item One" />
                            <Tab label="Item Two" />
                        </Tabs>
                    </Box>
                    <TabPanel value={0} index={0}>
                        <Box>
                            <Typography variant="inputLabel">
                                Connected with
                            </Typography>
                            <Badge marginLeft={theme.spacing(1)} label="Polkadot.js" icon={<Image src={"networks/polkadot_js.svg"} height="16px" width="16px" />} color={theme.palette.text.primary} background={alpha(theme.palette.text.primary, 0.1)} />
                        </Box>
                        <Box marginTop={theme.spacing(2)} display="flex" alignItems={"center"}>
                            <Image height="32" width="32" src="networks/polkadot_js.svg"></Image>

                            <Box marginLeft={theme.spacing(1)}>
                                <Box>
                                    <Typography variant="inputLabel">0xSlenderman</Typography>
                                </Box>
                                <Box>
                                    <Typography color={theme.palette.text.secondary} variant="caption">15Ve3dFDE4xRcWTPKPB4HDv8Asnpbha67iwr81WxjRJ1HHmq</Typography>
                                </Box>
                            </Box>
                        </Box>
                        <Box marginTop={theme.spacing(2)}>
                            <Typography color={theme.palette.text.secondary} variant="inputLabel">
                                Disconnect
                            </Typography>
                        </Box>
                    </TabPanel>
                    <TabPanel value={0} index={0}>
                        <Box>
                            <Typography variant="inputLabel">
                                Connected with Metamask
                            </Typography>
                            <Badge marginLeft={theme.spacing(1)} label="Polkadot.js" icon={<Image src={"networks/metamask_wallet.svg"} height="16px" width="16px" />} color={theme.palette.text.primary} background={alpha(theme.palette.text.primary, 0.1)} />
                        </Box>
                        <Box marginTop={theme.spacing(2)} display="flex" alignItems={"center"}>
                            <Image height="32" width="32" src="networks/mainnet.svg"></Image>
                            <Typography marginLeft={theme.spacing(1)} variant="inputLabel">0xf6ce427c29746936C954a30e892e829b65C0b22E</Typography>
                        </Box>
                        <Box marginTop={theme.spacing(2)}>
                            <Typography color={theme.palette.text.secondary} variant="inputLabel">
                                Disconnect
                            </Typography>
                        </Box>

                    </TabPanel>
                </Grid>
            </Grid>
        </Dialog>
    );
};
