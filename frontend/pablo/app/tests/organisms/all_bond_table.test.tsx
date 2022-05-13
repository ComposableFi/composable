import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/organisms/all_bond_table.stories";

const { Default } = composeStories(stories);

test("renders AllBondTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Pools")).toBeTruthy();
  expect(screen.queryAllByText("TVL")).toBeTruthy();
  expect(screen.queryAllByText("ROI")).toBeTruthy();
  expect(screen.queryAllByText("Rewards Left")).toBeTruthy();
  expect(screen.queryAllByText("Volume")).toBeTruthy();
});
