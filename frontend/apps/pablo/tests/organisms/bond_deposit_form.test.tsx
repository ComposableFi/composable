import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/bond_deposit_form.stories";

const { Default } = composeStories(stories);

test("renders Bond/DepositForm with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Bond")).toBeTruthy();
});
