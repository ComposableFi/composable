import { getNetwork, NETWORK_IDS } from "@/defi/Networks";
import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/network_tabs.stories"; // import all stories from the stories file

const { DefaultNetworkTabs } = composeStories(stories);

test("renders Network Tabs", () => {
  render(<DefaultNetworkTabs />);
  expect(screen.getByText(getNetwork(NETWORK_IDS[0]).name)).toBeInTheDocument();
  expect(screen.getByText(getNetwork(NETWORK_IDS[1]).name)).toBeInTheDocument();
  expect(screen.getByText(getNetwork(NETWORK_IDS[2]).name)).toBeInTheDocument();
});
