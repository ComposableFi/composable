import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-pablo/storybook/stories/organisms/unverified_pool_warning_modal.stories";

const { Default } = composeStories(stories);

test("renders UnverifiedPoolWarningModal with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Warning")).toBeTruthy();
});
