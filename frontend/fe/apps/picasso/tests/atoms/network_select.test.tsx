import { NETWORK_IDS } from "@/defi/Networks";
import { SUBSTRATE_NETWORK_IDS } from "@/defi/polkadot/Networks";
import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/network_select.stories"; // import all stories from the stories file

const { NetworkSelects, SubstrateNetworkSelect } = composeStories(stories);

describe("<NetworkSelect />", () => {
  test("Renders component with default networks", () => {
    const { container } = render(<NetworkSelects />);
    expect(
      container.querySelector(`input[value='${NETWORK_IDS[0]}']`)
    ).toBeDefined();
    expect(container.querySelector("input[disabled]")).toBeDefined();
  });

  test("Renders component with substrate networks", () => {
    const { container } = render(<SubstrateNetworkSelect />);
    expect(
      container.querySelector(`input[value='${SUBSTRATE_NETWORK_IDS[0]}']`)
    ).toBeDefined();
    expect(container.querySelector("input[disabled]")).toBeDefined();
  });
});
