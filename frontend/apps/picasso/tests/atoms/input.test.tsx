import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/input.stories"; // import all stories from the stories file

const {
  TextOnly,
  TextInsideButton,
  TokenInsideButton,
  TextAndReference,
  LabeledInputsWithBalance,
} = composeStories(stories);

test("renders Input/Text Only Input", () => {
  render(<TextOnly />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Alert text")).toBeInTheDocument();
});

test("renders Input/Text Inside Button", () => {
  render(<TextInsideButton />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Alert text")).toBeInTheDocument();
});

test("renders Input/Token Inside Button", () => {
  render(<TokenInsideButton />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Alert text")).toBeInTheDocument();
});

test("renders Input/Text And Reference", () => {
  render(<TextAndReference />);
  expect(screen.getByDisplayValue("Input text")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("Placeholder text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Disabled text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Error text")).toBeInTheDocument();
  expect(screen.getByDisplayValue("Alert text")).toBeInTheDocument();
});

test("renders Labeled Input With Balance", () => {
  render(<LabeledInputsWithBalance />);
  expect(screen.getAllByText("Label master here")).toBeDefined();
  expect(screen.getAllByText("Balance:")).toBeDefined();
});
