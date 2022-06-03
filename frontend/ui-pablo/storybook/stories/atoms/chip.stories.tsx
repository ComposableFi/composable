import { SxProps, Box, Chip, ChipProps } from "@mui/material";
import HowToVoteIcon from "@mui/icons-material/HowToVote";
import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import BallotIcon from "@mui/icons-material/Ballot";
import { Story } from "@storybook/react";

const ChipStories = (props: ChipProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    maxWidth: "fit-content",
    marginBottom: "20px",
  };

  return (
    <Box sx={boxStyle}>
      <Chip {...props} />
    </Box>
  );
};

export default {
  title: "atoms/Chip",
  component: ChipStories,
  color: {
    options: ["info", "success", "error", "warning"],
  },
};

const Template: Story<typeof ChipStories> = (args) => <ChipStories {...args} />;

export const ChipTemplate = Template.bind({});

ChipTemplate.args = {
  label: "Info",
  color: "info",
  icon: <HowToVoteIcon />,
};

export const ChipSuccessTemplate = Template.bind({});

ChipSuccessTemplate.args = {
  label: "Success",
  color: "success",
  icon: <CheckIcon />,
};

export const ChipErrorTemplate = Template.bind({});

ChipErrorTemplate.args = {
  label: "Error",
  color: "error",
  icon: <CloseIcon />,
};

export const ChipWarningTemplate = Template.bind({});

ChipWarningTemplate.args = {
  label: "Warning",
  color: "warning",
  icon: <BallotIcon />,
};

export const ChipTextTemplate = Template.bind({});

ChipTextTemplate.args = {
  label: "Only Text",
};
