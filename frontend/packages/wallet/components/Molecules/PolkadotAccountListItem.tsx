import React from "react";
import { alpha, Box, Button, Typography, useTheme } from "@mui/material";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import Identicon from '@polkadot/react-identicon';

export const PolkadotAccountListItem = ({ account, onSelect, isSelected, identiconTheme = "polkadot" }: {
    account: InjectedAccountWithMeta;
    onSelect: (account: InjectedAccountWithMeta) => void;
    isSelected: boolean;
    identiconTheme?: "substrate" | "polkadot" | "ethereum" | "jdenticon"
}) => {
    const theme = useTheme();
    return (
        <Button
            key={account.address}
            variant="outlined"
            color="primary"
            size="large"
            fullWidth
            onClick={() => {
                onSelect(account)
            }}
            sx={{
                height: "6.375rem",
                backgroundColor:
                    isSelected
                        ? alpha(theme.palette.primary.main, 0.1)
                        : "",
                display: "flex",
                justifyContent: "flex-start",
                alignItems: "center",
                gap: theme.spacing(2)
            }}
        >
            <Box sx={{ marginLeft: theme.spacing(1.75), marginTop: theme.spacing(0.5) }}>
                <Identicon
                    value={account.address}
                    size={24}
                    theme={identiconTheme}
                />
            </Box>
            <Box>
                <Typography textAlign={"left"}>{account.meta.name ?? account.address}</Typography>
                <Typography sx={{ display: { xs: 'none', sm: 'block' } }} textAlign={"left"} variant="inputLabel" color="text.secondary">
                    {account.address}
                </Typography>
            </Box>
        </Button>
    )
}