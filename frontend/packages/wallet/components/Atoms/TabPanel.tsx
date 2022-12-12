import { Box, Card, CardProps, useTheme } from "@mui/material";
import { FC } from "react";

export interface TabPanelProps extends CardProps {
  index: number;
  value: number;
}

export const TabPanel: FC<TabPanelProps> = ({
  children,
  value,
  index,
  ...other
}) => {
  const theme = useTheme();
  const { sx, ...cardProps } = other;
  return (
    <Card
      sx={{
        margin: theme.spacing(2, 0),
        ...sx,
      }}
      role="tabpanel"
      hidden={value !== index}
      id={`tabpanel-${index}`}
      aria-labelledby={`tab-${index}`}
      {...cardProps}
    >
      {value === index && <Box sx={{ p: 1 }}>{children}</Box>}
    </Card>
  );
};
