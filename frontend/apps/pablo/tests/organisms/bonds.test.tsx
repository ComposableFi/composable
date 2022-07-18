import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bonds.stories";

const { Default } = composeStories(stories);

test("renders Bonds with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Your active bonds")).toBeTruthy();
  expect(screen.queryAllByText("All bonds")).toBeTruthy();
});
