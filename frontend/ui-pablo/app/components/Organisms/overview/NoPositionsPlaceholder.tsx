import { Typography, Box } from "@mui/material";
import Image from "next/image";

export type NoPositionsProps = {
  text: string;
};

export const NoPositionsPlaceholder = ({ text }: NoPositionsProps) => {
  return (
    <Box textAlign="center" mt={3}>
      <Image
        src="/static/lemonade.png"
        css={{ mixBlendMode: "luminosity" }}
        width="96"
        height="96"
        alt="lemonade"
      />
      <Typography variant="body2" paddingTop={4} color="text.secondary">
        {text}
      </Typography>
    </Box>
  );
};
