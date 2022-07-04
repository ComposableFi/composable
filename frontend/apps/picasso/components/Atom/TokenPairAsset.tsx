import React from "react";
import { getToken, TokenId } from "tokens";
import { Box, BoxProps, Typography } from "@mui/material";
import Image from "next/image";

export type TokenPairAssetProps = {
  tokenIds: TokenId[];
  iconSize?: number;
  iconOnly?: boolean;
  centeredLabel?: boolean;
  label?: string;
} & BoxProps;

export const TokenPairAsset: React.FC<TokenPairAssetProps> = ({
  tokenIds,
  iconSize,
  iconOnly,
  centeredLabel,
  label,
  ...rest
}) => {
  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent={centeredLabel ? "center" : undefined}
      position="relative"
      width="100%"
      gap={iconOnly ? 0 : 2}
      flex="none"
      {...rest}
    >
      {tokenIds.length > 0 && (
        <Box
          display="flex"
          position={centeredLabel ? "absolute" : undefined}
          left={centeredLabel ? 0 : undefined}
          alignItems="center"
        >
          {tokenIds.map((tokenId, index) => (
            <Box
              key={tokenId}
              display="flex"
              marginLeft={index > 0 ? -1.5 : undefined}
            >
              <Image
                src={getToken(tokenId).icon}
                alt={getToken(tokenId).symbol}
                width={iconSize}
                height={iconSize}
              />
            </Box>
          ))}
        </Box>
      )}
      <Typography variant="body2" color="text.primary">
        {label ||
          `${tokenIds.map(tokenId => getToken(tokenId).symbol).join("-")}`}
      </Typography>
    </Box>
  );
};

TokenPairAsset.defaultProps = {
  iconSize: 24
};
