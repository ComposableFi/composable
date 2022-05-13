import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/switch.stories";

const { SwitchTemplate } = composeStories(stories);

test("renders <SwitchTemplate /> properly", () => {
  const { container } = render(<SwitchTemplate />);

  expect(container.getElementsByClassName("MuiSwitch-root").length).toBe(1);
});
