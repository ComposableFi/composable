# Code Reviews

Whether you're contributing or reviewing pull requests, this document is important to you!

- If you're reviewing, it can help you understand what to do.
- If you're submitting a pull request, it can help you understand what folks are going to do, how to help them, and how to get your pull request merged faster.

Ideas in this document are taken from [Google's Engineering Practices](https://google.github.io/eng-practices/review/). If this is your first time reading the document, start there.

# Creating a PR

- Make sure you read the [contributing guidelines](https://github.com/ComposableFi/composable/blob/main/docs/CONTRIBUTING.md). 

- Always branch off of `main` for both feature (`feat`) and `bugfix` branches. If you are working together with other developers on a feature and want to each have your your branch, create a `feat/{feature}/{GITHUB_USERNAME}` branch. Once the `feat/{feature}` branch has been deleted, we may delete all nested branches(!).

- Commits must be signed, as well as that all code should pass our automated checks and tests. Code will not be reviewed until all checks succeed. 

- Fill in the PR template, and make sure to describe your changes well. The developer reviewing your work may be out of the loop on the need of the change, and unfamiliar with the product itself. Linking to a ticket or issue is also acceptable.

- Do not request reviews from too many people. If you are in doubt, let the code owners review. If you specifically know the experts involved, feel free to request up to 3 reviewers.

# For Reviewers

- Be responsive. If you are requested to review, wait at most one business day. Even better to review within the first 4 hours. A review usually is a bit of back and forth, so a 4 hour response time will still lead to a PR taking 2 days to merge.

- For typos and small changes, use the `suggest change` feature.

- Reviewers have an educational function too. Style or small remarks are useful for learnings, but make sure to use `Nit:` as a prefix for the comment, to indicate that the remark is non-essential to the merge of the PR.

In order of importance for code quality:

1. Correctness: It does what it is supposed to do.
2. Test Coverage: We know that it is correct.
3. Interfaces and architecture: The signature of a function is more important than it's contents.
4. Test Brittleness: Good tests verify correctness and public interfaces, and are easy to refactor when the public interface changes

# Handling Feedback

Explanations written only in the code review tool are not helpful to future code readers. They are acceptable only in a few circumstances, such as when you are reviewing an area you are not very familiar with and the developer explains something that normal readers of the code would have already known. Unless the reviewer's comment was completely incorrect, any non-`nit` comments should result in a new commit fixing the code, or adding a comment describing the code.