import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/feeDisplay.stories";

const { FeeDisplayWithoutTooltip, FeeDisplayWithTooltip } =
  composeStories(stories);

describe("FeeDisplay", () => {
  test("renders <FeeDisplayWithoutTooltip /> properly", () => {
    render(<FeeDisplayWithoutTooltip />);

    expect(screen.getByText("Fee")).toBeInTheDocument();
    expect(screen.queryByTestId("InfoOutlinedIcon")).not.toBeInTheDocument();
    expect(screen.getByText("345 ETH")).toBeInTheDocument();
  });

  test("renders <FeeDisplayWithTooltip /> properly", () => {
    render(<FeeDisplayWithTooltip />);

    expect(screen.getByText("Fee")).toBeInTheDocument();
    expect(screen.getByTestId("InfoOutlinedIcon")).toBeInTheDocument();
    expect(screen.getByText("345 ETH")).toBeInTheDocument();
  });
});
