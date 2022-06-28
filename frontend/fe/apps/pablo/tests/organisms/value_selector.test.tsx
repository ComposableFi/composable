import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/value_selector.stories";

const { Default } = composeStories(stories);

test("renders ValueSelector with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("25%")).toBeTruthy();
});
