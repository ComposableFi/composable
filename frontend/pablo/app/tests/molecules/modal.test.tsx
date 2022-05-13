import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/molecules/modal.stories"; // import all stories from the stories file

const { FullScreenModal } = composeStories(stories);

test("renders <FullScreenModal /> properly", () => {
  render(<FullScreenModal />);

  expect(screen.getByText("This h1 will go as header")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "close" })).toBeInTheDocument();
});
