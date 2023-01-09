import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/network_select.stories";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";
import config from "@/constants/config"; // import all stories from the stories file

const { NetworkSelects, SubstrateNetworkSelect } = composeStories(stories);

describe("<NetworkSelect />", () => {
  test("Renders component with default networks", () => {
    const { container } = render(<NetworkSelects />);
    expect(
      container.querySelector(`input[value='${config.evm.networkIds[0]}']`)
    ).toBeDefined();
    expect(container.querySelector("input[disabled]")).toBeDefined();
  });

  test("Renders component with substrate networks", () => {
    const { container } = render(<SubstrateNetworkSelect />);
    expect(
      container.querySelector(`input[value='${SUBSTRATE_NETWORKS.picasso}']`)
    ).toBeDefined();
    expect(container.querySelector("input[disabled]")).toBeDefined();
  });
});
