import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "pablo-storybook/stories/organisms/overview_xpablos_box.stories";

const { Default } = composeStories(stories);

test("renders Overview/XPablosBox with default args", () => {
  render(<Default />);

  expect(screen.queryAllByText("Your xPBLO")).toBeTruthy();
  expect(screen.queryAllByText("fNFT ID")).toBeTruthy();
  expect(screen.queryAllByText("PBLO locked")).toBeTruthy();
  expect(screen.queryAllByText("Expiry")).toBeTruthy();
  expect(screen.queryAllByText("Multiplier")).toBeTruthy();
  expect(screen.queryAllByText("xPBLO")).toBeTruthy();
});
