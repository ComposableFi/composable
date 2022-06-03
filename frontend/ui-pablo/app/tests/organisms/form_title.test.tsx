import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-pablo/storybook/stories/organisms/form_title.stories";

const { Default } = composeStories(stories);

test("renders AllBondTable with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Form Title")).toBeTruthy();
});
