import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/tabs.stories"; // import all stories from the stories file

const { TextOnlyTabs, IconTabs } = composeStories(stories);

test("renders Text Only Tabs", () => {
  render(<TextOnlyTabs />);
  expect(screen.getByText("Tab 1")).toBeInTheDocument();
  expect(screen.getByText("Tab 2")).toBeInTheDocument();
  expect(screen.getByText("Tab 3")).toBeInTheDocument();
});

test("renders Icon Tabs", () => {
  render(<IconTabs />);
  expect(screen.getByText("Tab 1")).toBeInTheDocument();
  expect(screen.getByText("Tab 2")).toBeInTheDocument();
  expect(screen.getByText("Tab 3")).toBeInTheDocument();
});
