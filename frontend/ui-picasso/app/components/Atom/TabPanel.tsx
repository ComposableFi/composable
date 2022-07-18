import * as React from "react";
import { Card, useTheme, Box, Typography } from "@mui/material";

export interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

export const TabPanel: React.FC<TabPanelProps> = ({
  children,
  value,
  index,
  ...other
}) => {
  const theme = useTheme();
  return (
    <Card
      sx={{
        margin: theme.spacing(6, 0),
      }}
      role="tabpanel"
      hidden={value !== index}
      id={`tabpanel-${index}`}
      aria-labelledby={`tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </Card>
  );
};
