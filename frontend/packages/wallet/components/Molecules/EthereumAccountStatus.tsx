import { Box, Button } from "@mui/material";
import React from "react";
import { Input } from "../Atoms/Input";

export const EthereumAccountStatus = ({
    connectedAddress,
    handleEthereumDisconnect
}: { handleEthereumDisconnect: () => void; connectedAddress: string }) => {
    return (
        <Box width="100%">
            <Input
                value={connectedAddress.toLowerCase()}
                disabled
                fullWidth
                sx={{
                    mt: 8,
                }}
                inputProps={{
                    inputProps: {
                        sx: {
                            textAlign: "center",
                        },
                    },
                }}
            />
            <Button
                fullWidth
                variant="text"
                size="large"
                onClick={() => handleEthereumDisconnect()}
                sx={{ mt: 4 }}
            >
                Disconnect wallet
            </Button>
        </Box>
    )
}
