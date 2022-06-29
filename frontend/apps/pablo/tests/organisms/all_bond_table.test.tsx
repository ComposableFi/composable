import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/all_bond_table.stories";

const { Default } = composeStories(stories);

test("renders AllBondTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Asset")).toBeTruthy();
  expect(screen.queryAllByText("Price")).toBeTruthy();
  expect(screen.queryAllByText("ROI")).toBeTruthy();
  expect(screen.queryAllByText("Total purchased")).toBeTruthy();
});
