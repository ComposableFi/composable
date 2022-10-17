import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/token_dropdown_combined_input.stories"; // import all stories from the stories file

const { TokenDropdownCombinedInputs, TokenDropdownCombinedInputsWithButton } =
  composeStories(stories);

test("renders Input/Token Dropdown Combined", () => {
  render(<TokenDropdownCombinedInputs />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});

test("renders Input/Token Dropdown Combined with Button", () => {
  render(<TokenDropdownCombinedInputsWithButton />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
});
