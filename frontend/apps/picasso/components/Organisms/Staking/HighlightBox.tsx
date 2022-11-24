import { alpha, Box, Typography, useTheme } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FeaturedBox } from "@/components";
import { FC } from "react";

type HighlighBoxProps = {
  title: string;
  tooltip: string;
  value: string;
  disabled?: boolean;
};

export const HighlightBox: FC<HighlighBoxProps> = ({
  title,
  tooltip,
  value,
  disabled,
}) => {
  const theme = useTheme();
  return (
    <FeaturedBox
      textAbove={
        <Box display="flex" alignItems="center" justifyContent="center">
          <TextWithTooltip
            TypographyProps={{
              color: alpha(theme.palette.common.white, 0.6),
            }}
            tooltip={tooltip}
            disabled={disabled}
          >
            {title}
          </TextWithTooltip>
        </Box>
      }
      textBelow={<Typography variant="h6">{value}</Typography>}
    />
  );
};
