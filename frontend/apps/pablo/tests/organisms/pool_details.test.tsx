import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/pool_details.stories";

const { Default } = composeStories(stories);

test("renders PoolDetails with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Pool value")).toBeTruthy();
  expect(screen.queryAllByText("Rewards left")).toBeTruthy();
  expect(screen.queryAllByText("Volume (24H)")).toBeTruthy();
  expect(screen.queryAllByText("Fees (24H)")).toBeTruthy();
  expect(screen.queryAllByText("APR")).toBeTruthy();
  expect(screen.queryAllByText("Transactions (24H)")).toBeTruthy();
  expect(screen.queryAllByText("Liquidity")).toBeTruthy();
  expect(screen.queryAllByText("Staking")).toBeTruthy();
  expect(screen.queryAllByText("Rewards")).toBeTruthy();
});
