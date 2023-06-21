import { 
  FeaturedBox, 
  FeaturedBoxProps 
} from "picasso/components";
import { 
  Box, 
  SxProps
} from "@mui/material";
import { Story } from "@storybook/react";

const FeaturedBoxStories = (props: FeaturedBoxProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 4,
  };

  return (
    <Box sx={boxStyle}>
      <FeaturedBox {...props} />
    </Box>
  );
};

export default {
  title: "molecules/FeaturedBox(General)",
  component: FeaturedBox,
};

// Default arguments for all stories
const defaultArgs = {
  title: 'Text Title',
  textAbove: 'Text Above',
  textBelow: 'Text Below',
};

const Template: Story<typeof FeaturedBoxStories> = (args) => (
  <FeaturedBoxStories {...args} {...defaultArgs} />
);

// CTA Featured Box
export const CTAFeaturedBox = Template.bind({});
CTAFeaturedBox.args = {
  ButtonProps: {
    label: 'Master button',
    onClick: () => {},
  },
};

// Horizontal Aligned CTA Featured Box
export const HorizontalAlignedCTAFeaturedBox = Template.bind({});
HorizontalAlignedCTAFeaturedBox.args = {
  ButtonProps: {
    label: 'Master button',
    onClick: () => {},
  },
  horizontalAligned: true,
};

// CTA Featured Box with full width action button
export const CTAFeaturedBoxWithFullWidthActionButton = Template.bind({});
CTAFeaturedBoxWithFullWidthActionButton.args = {
  ButtonProps: {
    label: 'Master button',
    onClick: () => {},
    fullWidth: true,
  },
};

// CTA featured Box WITH contained action button
export const CTAFeaturedBoxWithContainedActionButton = Template.bind({});
CTAFeaturedBoxWithContainedActionButton.args = {
  ButtonProps: {
    label: 'Master button',
    onClick: () => {},
    variant: 'contained',
  },
};

// Outlined CTA featured Box WITH contained action button
export const OutlinedCTAFeaturedBox = Template.bind({});
OutlinedCTAFeaturedBox.args = {
  variant: 'outlined',
  ButtonProps: {
    label: 'Master button',
    onClick: () => {},
    variant: 'contained',
  },
  horizontalAligned: true,
};

// Token Featured Box
export const TokenFeaturedBox = Template.bind({});
TokenFeaturedBox.args = {
  token: {
    icon: '/tokens/eth-mainnet.svg',
    symbol: 'ETH'
  }, 
};
