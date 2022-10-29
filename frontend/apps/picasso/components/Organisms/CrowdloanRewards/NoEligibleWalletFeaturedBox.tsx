import { Box, Grid, useTheme } from "@mui/material";
import { FeaturedBox } from "../../Molecules";
import Image from "next/image";

export const NoEligibleWalletFeaturedBox: React.FC<{
  title: string;
  textBelow: string;
}> = ({ title, textBelow }) => {
  const theme = useTheme();
  return (
    <Grid item xs={12} mt={theme.spacing(2)}>
      <FeaturedBox
        textAbove={title}
        TextAboveProps={{
          color: "white",
        }}
        textBelow={textBelow}
        image={
          <Image
            alt="lemonade"
            css={{ mixBlendMode: "luminosity" }}
            src="/static/lemonade.png"
            width="96"
            height="96"
          />
        }
      />
    </Grid>
  );
};
