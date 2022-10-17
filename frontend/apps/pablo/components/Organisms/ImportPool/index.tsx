import { getToken, TOKENS } from "@/defi/Tokens";
import { useMobile } from "@/hooks/responsive";
import {
  Box,
  Button,
  MenuItem,
  Select,
  SelectChangeEvent,
  Typography,
  useTheme,
} from "@mui/material";
import { useRouter } from "next/router";
import { FormTitle } from "@/components/Organisms";
import { Link } from "@/components/Molecules";
import React, { useState } from "react";
import { Token, TokenId } from "@/defi/types";
import { BoxProps } from "@mui/system";
import { useDispatch } from "react-redux";
import {
  openPolkadotModal,
  openTransactionSettingsModal,
} from "@/stores/ui/uiSlice";
import Image from "next/image";
import { PairAsset } from "@/components/Atoms";
import { InfoOutlined } from "@mui/icons-material";
import { TransactionSettings } from "../TransactionSettings";
import { useDotSamaContext } from "substrate-react";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

const options = Object.values(TOKENS);
export const ImportPool: React.FC<BoxProps> = ({ ...rest }) => {
  const { extensionStatus } = useDotSamaContext();
  const isMobile = useMobile();
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();
  const [token1Value, setToken1Value] = useState(options[0].id);
  const [token2Value, setToken2Value] = useState<TokenId>();
  const [isLiquidityPresent, setIsLiquidityPreset] = useState(false);

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  return (
    <HighlightBox width={isMobile ? "100%" : 550} margin="auto" {...rest}>
      <FormTitle
        title="Import Pool"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Typography variant="subtitle1" textAlign="center" mt={4}>
        {`Use this tool to find pairs that don't automatically appear in the
        interface.`}
      </Typography>

      <Box mt={4}>
        <Select
          onChange={(e: SelectChangeEvent) => {
            const newValue = e.target.value as TokenId;
            setToken1Value(newValue);
          }}
          value={token1Value}
          fullWidth
        >
          {options.map((token: Token) => {
            return (
              <MenuItem key={token.id} value={token.id}>
                <Box gap={1} display="flex" alignItems="center">
                  <Image
                    src={token.icon}
                    alt={token.id}
                    width={18}
                    height={18}
                  />
                  {token.symbol}
                </Box>
              </MenuItem>
            );
          })}
        </Select>
      </Box>

      <Box mt={4} textAlign="center">
        <Box
          width={56}
          height={56}
          borderRadius="50%"
          display="flex"
          border={`2px solid ${theme.palette.primary.main}`}
          justifyContent="center"
          alignItems="center"
          margin="auto"
        >
          <Typography variant="h5">+</Typography>
        </Box>
      </Box>

      <Box mt={4}>
        <Select
          onChange={(e: SelectChangeEvent) => {
            const newValue = e.target.value as TokenId;
            setToken2Value(newValue);
          }}
          label="Select a token"
          value={token2Value}
          fullWidth
        >
          {options.map((token: Token) => {
            return (
              <MenuItem key={token.id} value={token.id}>
                <Box gap={1} display="flex" alignItems="center">
                  <Image
                    src={token.icon}
                    alt={token.id}
                    width={18}
                    height={18}
                  />
                  {token.symbol}
                </Box>
              </MenuItem>
            );
          })}
        </Select>
      </Box>

      <Box mt={4}>
        {extensionStatus !== "connected" && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            onClick={() => {
              dispatch(openPolkadotModal());
            }}
          >
            Connect wallet to find pools
          </Button>
        )}

        {extensionStatus === "connected" &&
          token1Value &&
          token2Value &&
          isLiquidityPresent && (
            <Box mt={6} mb={4}>
              <Select
                sx={{
                  backgroundColor: theme.palette.primary.dark,
                  "> .MuiOutlinedInput-notchedOutline": {
                    border: "none",
                  },
                }}
                value={0}
                fullWidth
                disabled
              >
                <MenuItem value={0}>
                  <PairAsset
                    assets={[
                      {
                        icon: getToken(token1Value).icon,
                        label: getToken(token1Value).symbol,
                      },
                      {
                        icon: getToken(token2Value).icon,
                        label: getToken(token2Value).symbol,
                      },
                    ]}
                    separator="/"
                  />
                </MenuItem>
              </Select>
            </Box>
          )}

        {extensionStatus === "connected" && isLiquidityPresent && (
          <Button variant="contained" size="large" fullWidth>
            Import pool
          </Button>
        )}

        {extensionStatus === "connected" && !isLiquidityPresent && (
          <Box
            display="flex"
            flexDirection="column"
            alignItems="center"
            justifyContent="center"
          >
            <Box
              width="100%"
              padding={2}
              borderRadius={1}
              sx={{ background: theme.palette.warning.dark }}
              mb={2}
              justifyContent="center"
              alignItems="center"
              gap={1}
              display="flex"
            >
              <InfoOutlined sx={{ color: theme.palette.warning.main }} />
              <Typography color={theme.palette.warning.main}>
                {`You don't have liquidity in this pool yet.`}
              </Typography>
            </Box>
            <Link href="/pool/add-liquidity" key="import">
              <Typography
                textAlign="center"
                variant="subtitle1"
                color="primary"
                sx={{ cursor: "pointer" }}
                mt={1}
              >
                Add liquidity
              </Typography>
            </Link>
          </Box>
        )}
        <TransactionSettings />
      </Box>
    </HighlightBox>
  );
};
