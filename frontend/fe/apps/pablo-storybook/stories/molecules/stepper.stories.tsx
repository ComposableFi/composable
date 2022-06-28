import { ComponentStory } from "@storybook/react";
import { Stepper } from "pablo/components";

export default {
  title: "molecules/Stepper",
  component: Stepper,
};

const steps = [
  'Step 1',
  'Step 2',
  'Step 3',
  'Step 4',
];

const Template: ComponentStory<typeof Stepper> = (args) => (
  <Stepper {...args} />
);

export const StepperStory = Template.bind({});
StepperStory.args = {
  currentStep: 3,
  steps: steps,
};
