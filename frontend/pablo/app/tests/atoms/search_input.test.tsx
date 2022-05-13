import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "@ui-template/storybook/stories/atoms/search_input.stories"; // import all stories from the stories file

const { SearchInputs } = composeStories(stories);

test("renders Search Input", () => {
  const { container } = render(<SearchInputs />);
  expect(container.querySelector("input[value='Search text']")).toBeDefined();
  expect(
    container.querySelector("input[placeholder='Placeholder text']")
  ).toBeDefined();
  expect(container.querySelector("input[value='Disabled text']")).toBeDefined();
});
