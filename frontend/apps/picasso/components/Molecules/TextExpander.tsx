import { Box, Fade } from "@mui/material";
import { FC, ReactElement, useState } from "react";

export const TextExpander: FC<{
  short: ReactElement;
  expanded: ReactElement;
}> = ({ short, expanded }) => {
  const [isExpanded, setExpanded] = useState(false);
  return (
    <Box
      sx={{
        cursor: "help",
      }}
      onMouseEnter={() => setExpanded(true)}
      onMouseLeave={() => setExpanded(false)}
    >
      {!isExpanded ? (
        <Fade in={!isExpanded} appear>
          {short}
        </Fade>
      ) : null}
      {isExpanded ? (
        <Fade in={isExpanded} appear>
          {expanded}
        </Fade>
      ) : null}
    </Box>
  );
};
