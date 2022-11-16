import { Box } from "@mui/material";
import { FC, ReactNode, useState } from "react";

export const TextExpander: FC<{
  short: ReactNode;
  expanded: ReactNode;
}> = ({ short, expanded }) => {
  const [isExpanded, setExpanded] = useState(false);
  return (
    <Box
      onMouseEnter={() => setExpanded(true)}
      onMouseLeave={() => setExpanded(false)}
    >
      {isExpanded ? expanded : short}
    </Box>
  );
};
