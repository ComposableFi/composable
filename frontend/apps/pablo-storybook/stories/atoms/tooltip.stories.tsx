import { Story } from "@storybook/react";
import IconButton from "@mui/material/IconButton";
import { useTheme } from "@mui/material/styles";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import MUITooltip, { TooltipProps } from "@mui/material/Tooltip";
import InfoOutlineRoundedIcon from "@mui/icons-material/InfoOutlined";

const TooltipStories = (props: TooltipProps) => {
  const theme = useTheme();
  return (
    <Box display="flex" alignItems="center">
      <Typography mr="1rem">Available to claim</Typography>
      <MUITooltip {...props} arrow>
        <IconButton>
          <InfoOutlineRoundedIcon
            sx={{
              color: theme.palette.primary.light,
            }}
          />
        </IconButton>
      </MUITooltip>
    </Box>
  );
};

export default {
  title: "atoms/Tooltip",
  component: TooltipStories,
};

const Template: Story<typeof TooltipStories> = (args) => (
  // @ts-ignore
  <TooltipStories {...args} />
);

export const Tooltips = Template.bind({});

Tooltips.args = {
  arrow: true,
  title: "Text element"
};