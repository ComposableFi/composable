import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/molecules/dropdown_combined_input.stories"; // import all stories from the stories file

const {
  DropdownCombinedInputs,
  DropdownCombinedInputsWithButton,
  IconDropdownCombinedInputs,
  IconDropdownCombinedInputsWithButton,
} = composeStories(stories);

test("renders Input/Dropdown Combined", () => {
  render(<DropdownCombinedInputs />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});

test("renders Input/Dropdown Combined with Button", () => {
  render(<DropdownCombinedInputsWithButton />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});

test("renders Input/Icon Dropdown Combined", () => {
  render(<IconDropdownCombinedInputs />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});

test("renders Input/Icon Dropdown Combined with Button", () => {
  render(<IconDropdownCombinedInputsWithButton />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});
