import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/staking_renew_modal.stories";

const { Default } = composeStories(stories);

test("renders Staking/RenewModal with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Renew staking period")).toBeTruthy();
  expect(screen.queryAllByText("Enter amount to stake")).toBeTruthy();
  expect(screen.queryAllByText("Select lock period")).toBeTruthy();
  expect(screen.queryAllByText("Unlock date")).toBeTruthy();
  expect(screen.queryAllByText("Renew period")).toBeTruthy();
});
