import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-pablo/storybook/stories/organisms/staking.stories";

const { Default } = composeStories(stories);

test("renders Staking with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Stake")).toBeTruthy();
  expect(screen.queryAllByText("Lock PBLO to mint CHAOS, the yield bearing governance fNFT.")).toBeTruthy();
  expect(screen.queryAllByText("Stake and mint")).toBeTruthy();
  expect(screen.queryAllByText("Burn and unstake")).toBeTruthy();
});
