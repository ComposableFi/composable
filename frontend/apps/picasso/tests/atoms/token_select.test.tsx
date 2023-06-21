import { TOKEN_IDS } from "tokens";
import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/token_select.stories"; // import all stories from the stories file

const { TokenSelects } = composeStories(stories);

test("renders Token Selects", () => {
  const { container } = render(<TokenSelects />);
  expect(
    container.querySelector(`input[value='${TOKEN_IDS[0]}']`)
  ).toBeDefined();
  expect(container.querySelector("input[disabled]")).toBeDefined();
});
