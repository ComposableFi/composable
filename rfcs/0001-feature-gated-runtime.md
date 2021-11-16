# Summary

This RFS proposes a solution to maintain several development streams and runtimes in a single branch.

# Motivation


Usually, there is an approach to developing software in branches. I argue that this complicates things in the current state and makes our progress slower and more brittle.

I suggest that using a conditional compilation of runtime to achieve several streams of development without losing the security qualities of the codebase is way to make development easier until runtime is released to Polkadot.


# Detailed design

I suggest to turn on and off runtime features which in development via develop.


###

### IDE support

Rust analyzer works well with features behind `rust-analyzer.cargo.features` configuration in VS Code. It is clearly visible what features are on and off.

### Statistics gathered


# Alternatives

Using develop branch is an attempted alternative to go forward. I see next problems with it as of now:

- When I need code in main which is in develop in vice versa, I cannot use what is required as dependency of my task.
- Not no strict guidelines on what should go to main and what should go into development.
- Merging main into develop and develop is hard and error-prone. Can introduce security bugs.
- Develop is not uptoday with same GitOps and linting practies into main. Which leads to broken and hard to fix build issues, like inability to build runtime on  current develop.

# Unresolved questions

