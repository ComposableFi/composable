import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_unstake_modal.stories";
import { Provider } from "react-redux";
import { store } from "@/stores/root";

const { Default } = composeStories(stories);

test("renders Staking/RenewModal with default args", () => {
  render(<Provider store={store} ><Default /></Provider>);

  expect(screen.queryAllByText("Burn and unstake you position")).toBeTruthy();
  expect(screen.queryAllByText("Withdrawable PBLO")).toBeTruthy();
  expect(screen.queryAllByText("Initial PBLO deposit")).toBeTruthy();
  expect(screen.queryAllByText("Burn and unstake")).toBeTruthy();
});
