import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/recipientDropdown.stories";

const { DefaultRecipientDropdown } = composeStories(stories);

describe("RecipientDropdown", () => {
  test("renders <DefaultRecipientDropdown /> properly", () => {
    render(<DefaultRecipientDropdown />);

    expect(screen.getByText("Recipient")).toBeInTheDocument();
    expect(screen.getByText("Select 2")).toBeInTheDocument();
  });

  test("renders <DefaultRecipientDropdown /> in expanded state", () => {
    const { container } = render(<DefaultRecipientDropdown expanded={true} />);

    expect(
      container.querySelector(".MuiAccordion-root.Mui-expanded")
    ).not.toBeNull();
  });

  test("renders <DefaultRecipientDropdown /> in collapsed state", () => {
    const { container } = render(<DefaultRecipientDropdown expanded={false} />);

    expect(
      container.querySelector(".MuiAccordion-root.Mui-expanded")
    ).toBeNull();
  });
});
