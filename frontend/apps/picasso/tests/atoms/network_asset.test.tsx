import { getNetwork, NETWORK_IDS } from "@/defi/Networks";
import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/network_asset.stories"; // import all stories from the stories file

const { NetworkAssets } = composeStories(stories);

test("renders Network Asset With Default Args", () => {
  const networkId = NETWORK_IDS[0];
  const network = getNetwork(networkId);
  render(<NetworkAssets />);
  expect(screen.getByText(network.name)).toBeInTheDocument();
  expect(screen.getByAltText(network.name)).toBeInTheDocument();
});
