import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/overview_your_bonds_box.stories";

const { Default } = composeStories(stories);

test("renders Overview/YourBondsBox with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Your Bonds")).toBeTruthy();
  expect(screen.queryAllByText("Assets")).toBeTruthy();
  expect(screen.queryAllByText("Discount")).toBeTruthy();
  expect(screen.queryAllByText("Amount")).toBeTruthy();
  expect(screen.queryAllByText("Value")).toBeTruthy();
  expect(screen.queryAllByText("Vesting")).toBeTruthy();
  expect(screen.queryAllByText("Claimable")).toBeTruthy();
});
