import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/votingDetailsBox.stories"; // import all stories from the stories file

const { VotingDetailsBoxTemplate } = composeStories(stories);

test("renders Voting Details Box", () => {
  const { container } = render(<VotingDetailsBoxTemplate />);
  expect(screen.getByText("Proposal Title")).toBeInTheDocument();
  expect(screen.getByText("19d 21h 45m remaining")).toBeInTheDocument();
  expect(screen.getByText("Ecosystem")).toBeInTheDocument();
  expect(screen.getByText("Rejected")).toBeInTheDocument();
  expect(screen.getByText("by 12tb....432")).toBeInTheDocument();
  expect(
    container.getElementsByClassName("MuiChip-iconColorError").length
  ).toBe(1);
});
