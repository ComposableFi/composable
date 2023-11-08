# NEAR IBC
_IBC everywhere_

:::note

This section outlines our progress in connecting the IBC Protocol to NEAR and our upcoming plans. While we are still working on this project, we expect to launch sometime in Q1 2024.

:::

Composable’s bridging team have been working in close collaboration with the NEAR team to bring the necessary components for a GRANDPA light client implementation on [NEAR Protocol](https://near.org/). To do this, we have been assisting NEAR’s core developers with the development of infrastructure-level improvements for:

1. Signature verification
2. Missing validity
3. Singular validation process
4. And more

All of these can be found in our NEAR enhancement proposals [NEP-364](https://github.com/near/NEPs/pull/364), [NEP-384](https://github.com/near/NEPs/pull/384) and 9 PRs which have been merged into NEAR’s codebase. These changes will pave the way for our light client implementation, bringing IBC to the NEAR ecosystem. Additionally, we have received a grant from the NEAR Foundation as a result of Composable establishing an IBC bridge to NEAR Protocol.

The following is a diagram detailing Composable IBC's implementation to NEAR Protocol:

![centauri_stack](../images-centauri/NEAR-temp.png)

NEAR IBC architecture

We have plans to develop within ecosystems such as NEAR Protocol, with the ultimate goal of interconnecting all blockchains through IBC in the future.