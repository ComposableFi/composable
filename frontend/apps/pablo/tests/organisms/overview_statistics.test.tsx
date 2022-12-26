import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/overview_statistics.stories";

const { Default } = composeStories(stories);

test("renders Overview/Statistics with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Total value locked")).toBeTruthy();
  expect(screen.queryAllByText("24h trading volume")).toBeTruthy();
  expect(screen.queryAllByText("PBLO price")).toBeTruthy();
});
