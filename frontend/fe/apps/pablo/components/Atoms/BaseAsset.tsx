import React from "react";
import { Box, BoxProps, Typography, TypographyProps } from "@mui/material";
import Image from "next/image";

export type BaseAssetProps = {
  icon?: string;
  label?: string;
  LabelProps?: TypographyProps,
  iconSize?: number;
  centeredLabel?: boolean;
} & BoxProps;

export const BaseAsset: React.FC<BaseAssetProps> = ({
  icon,
  label,
  LabelProps,
  iconSize,
  centeredLabel,
  ...rest
}) => {
  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent={centeredLabel ? "center" : undefined}
      position="relative"
      gap={label ? 2 : 0}
      flex="none"
      {...rest}
    >
      {icon && (
        <Box
          display="flex"
          position={centeredLabel ? "absolute" : undefined}
          left={centeredLabel ? 0 : undefined}
          alignItems="center"
        >
          <Box display="flex">
            <Image
              src={icon}
              alt={label}
              width={iconSize}
              height={iconSize}
            />
          </Box>
        </Box>
      )}
      {label && (
        <Typography variant="body1" color="text.primary" {...LabelProps}>
          {label}
        </Typography>
      )}
    </Box>
  );
};

BaseAsset.defaultProps = {
  iconSize: 24,
};
