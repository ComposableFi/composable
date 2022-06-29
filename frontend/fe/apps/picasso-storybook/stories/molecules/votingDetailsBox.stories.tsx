import { Story } from "@storybook/react";
import { SxProps, Box } from "@mui/material";
import {
  VotingDetailsBox,
  VotingDetailsBoxProps,
} from "picasso/components/Molecules/VotingDetailsBox";
import CloseIcon from "@mui/icons-material/Close";

const VotingDetailsBoxStories = (props: VotingDetailsBoxProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    width: "fit-content",
    minWidth: "60%",
  };

  return (
    <Box sx={boxStyle}>
      <VotingDetailsBox {...props} />
    </Box>
  );
};

export default {
  title: "Organisms/VotingDetailsBox",
  component: VotingDetailsBoxStories,
};

const Template: Story<typeof VotingDetailsBoxStories> = (args) => (
  <VotingDetailsBoxStories {...args} />
);

export const VotingDetailsBoxTemplate = Template.bind({});

VotingDetailsBoxTemplate.args = {
  id: "12",
  title: "Proposal Title",
  status: "error",
  statusText: "Rejected",
  timeText: "19d 21h 45m remaining",
  statusIcon: <CloseIcon />,
  address: "12tb....432",
  tagText: "Ecosystem",
};
