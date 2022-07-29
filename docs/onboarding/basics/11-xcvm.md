# XCVM

`XCVM` (Cross-chain virtual machine) is an application-level protocol being built on top of IBC (and other trustless technology) by us. It's a bridge-agnostic subset of `XCM`, tailored for use by third-party application developers. 

The [specification](https://github.com/ComposableFi/composable/blob/main/xcvm/SPEC.md) should provide you with an understanding, although depending on your technical background, maybe a bit formal and complicated. Because the spec is still a work-in-progress, we haven't yet included a layman's explanation as part of your onboarding. 

### Takeaways

- `XCVM` takes the best of XCM and turns that into a more generalizable model for cross-chain applications.
- `IBC` and other bridges form a TCP-like backbone for the multichain ecosystem, but we still need sophisticated protocols on top of this backbone to build out the cross-chain future.

## Applications

By now you should have a decent understanding of bridging, cross-chain communication, and the different technologies out there. As an exercise, design a cross-chain application, combining the different techs you've learned about. Go as crazy as you like, but here are some ideas to get you started:

- A cross-chain lending application.
- Structured liquid-staking products.
- Chain-to-tradfi integrations.

Spend ~20 minutes formulating how you'd combine some tech to build something. If you feel overwhelmed, don't worry. You can also think about what is needed from a UI/UX standpoint, and what you feel is missing right now in crypto.