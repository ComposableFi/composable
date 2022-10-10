import { Box, Typography } from "@mui/material";
import Image from "next/image";
import { getImageURL } from "@/utils/nextImageUrl";

export type NoPositionsProps = {
  text: string;
};

export const NoPositionsPlaceholder = ({ text }: NoPositionsProps) => {
  return (
    <Box textAlign="center" mt={3}>
      <Image
        src={getImageURL("/static/lemonade.png")}
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
