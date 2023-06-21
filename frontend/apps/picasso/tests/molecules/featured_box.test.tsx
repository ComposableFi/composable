import { render, screen } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/molecules/featuredBox.stories"; // import all stories from the stories file

const {
  CTAFeaturedBox,
  HorizontalAlignedCTAFeaturedBox,
  CTAFeaturedBoxWithContainedActionButton,
  CTAFeaturedBoxWithFullWidthActionButton,
  OutlinedCTAFeaturedBox,
  TokenFeaturedBox,
} = composeStories(stories);

test("renders CTA Featured Box", () => {
  const { container } = render(<CTAFeaturedBox />);
  expect(container.getElementsByClassName("MuiButton-outlined").length).toBe(1);
});

test("renders Horizontal Aligned CTA Featured Box", () => {
  const { container } = render(<HorizontalAlignedCTAFeaturedBox />);
  expect(container.getElementsByClassName("MuiButton-outlined").length).toBe(1);
});

test("renders CTA Featured Box With Full Width Action Button", () => {
  const { container } = render(<CTAFeaturedBoxWithFullWidthActionButton />);
  expect(container.getElementsByClassName("MuiButton-fullWidth").length).toBe(
    1
  );
});

test("renders CTA Featured Box With Contained Action Button", () => {
  render(<CTAFeaturedBoxWithContainedActionButton />);

  expect(screen.queryByRole("button")).toBeTruthy();
});

test("renders Outlined CTA Featured Box", () => {
  render(<OutlinedCTAFeaturedBox />);
  expect(screen.queryByRole("button")).toBeTruthy();
});

test("renders Token Featured Box", () => {
  const { container } = render(<TokenFeaturedBox />);
  expect(container.getElementsByTagName("img").length).toBe(2);
});
