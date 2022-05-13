import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/linearProgress.stories";

const { LinearProgress } = composeStories(stories);

test("renders <LinearProgress /> properly", () => {
  const { container } = render(<LinearProgress />);

  expect(
    container.getElementsByClassName("MuiLinearProgress-root").length
  ).toBe(1);
});
