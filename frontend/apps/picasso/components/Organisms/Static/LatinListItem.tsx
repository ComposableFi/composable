import { BoxProps, useTheme } from "@mui/material";
import { ListItem } from "@/components/Organisms/Static/ListItem";

export const LatinListItem = ({ children }: BoxProps) => {
  const theme = useTheme();
  return (
    <ListItem
      sx={{
        listStyleType: "lower-latin",
        fontSize: theme.typography.body3.fontSize,
        lineHeight: "140%",
        my: 1,
      }}
    >
      {children}
    </ListItem>
  );
};
