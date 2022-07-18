import { Modal } from "@/components/Molecules/Modal";
import { Box, Button, Typography } from "@mui/material";
import { ComponentStory } from "@storybook/react";

const ModalStories = () => {
  return (
    <>
      {/* Text and colored boxes are added to showcase blur filter. */}
      <Typography variant="h1">
        This is rendered blurred in the background
        <Box
          sx={{ width: "300px", height: "300px", backgroundColor: "yellow" }}
        />
      </Typography>
      <Modal open={true} dismissible maxWidth="md">
        <Typography variant="h1">This h1 will go as header</Typography>
        <Typography variant="h5">This h5 will go as body</Typography>
        <Button variant="outlined" color="primary" fullWidth>
          And we can have buttons
        </Button>
      </Modal>
    </>
  );
};
export default {
  title: "molecules/Modal",
  component: Modal,
  argTypes: {
    open: {
      control: {
        type: "select",
        label: "Open",
        options: [true, false],
      },
    },
  },
};

const Template: ComponentStory<typeof Modal> = (args) => (
  <ModalStories {...args} />
);

export const FullScreenModal = Template.bind({});
FullScreenModal.args = {};
