import { TOKENS } from "tokens";
import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/molecules/tokenvalue.stories";

const { Default } = composeStories(stories);

test("renders Molecules/TokenValue with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText(TOKENS.pblo.symbol)).toBeTruthy();
  expect(screen.queryAllByText("500.00")).toBeTruthy();
});

