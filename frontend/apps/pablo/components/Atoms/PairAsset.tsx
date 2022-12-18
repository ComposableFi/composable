import React from "react";
import { Box, BoxProps, Typography, TypographyProps } from "@mui/material";
import Image from "next/image";
import { Asset } from "shared";

export type PairAssetProps = {
  assets: Asset[];
  iconSize?: number;
  iconOnly?: boolean;
  centeredLabel?: boolean;
  label?: string;
  LabelProps?: TypographyProps;
  separator?: string;
} & BoxProps;

export const PairAsset: React.FC<PairAssetProps> = ({
  assets,
  iconSize,
  iconOnly,
  centeredLabel,
  label,
  LabelProps,
  separator = "-",
  ...boxProps
}) => {
  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent={centeredLabel ? "center" : undefined}
      position="relative"
      gap={iconOnly ? 0 : 2}
      flex="none"
      {...boxProps}
    >
      {assets.length > 0 && (
        <Box
          display="flex"
          position={centeredLabel ? "absolute" : undefined}
          left={centeredLabel ? 0 : undefined}
          alignItems="center"
        >
          {assets.map((asset, index) => (
            <Box
              key={index}
              display="flex"
              marginLeft={index > 0 ? -1.5 : undefined}
            >
              <Image
                src={asset.getIconUrl()}
                alt={asset.getSymbol()}
                width={iconSize}
                height={iconSize}
              />
            </Box>
          ))}
        </Box>
      )}
      {!iconOnly && (
        <Typography variant="body2" color="text.primary" {...LabelProps}>
          {label ||
            `${assets.map((asset) => asset.getSymbol()).join(separator)}`}
        </Typography>
      )}
    </Box>
  );
};

PairAsset.defaultProps = {
  iconSize: 24,
};
