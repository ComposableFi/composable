
Required for merge:
- [ ] `pr-workflow-check / draft-release-check` is ✅ success
- Other rules GitHub shows you, or can be read in [configuration](github.com/ComposableFi/env/terraform/github.com/branches.tf) 

Makes review faster:
- [ ] PR title is my best effort to provide summary of changes and has clear text to be part of release notes 
- [ ] I marked PR by `misc` label if it should not be in release notes
- [ ] Linked Zenhub/Github/Slack/etc reference if one exists
- [ ] I was clear on what type of deployment required to release my changes (node, runtime, contract, indexer, on chain operation, frontend, infrastructure) if any in PR title or description
- [ ] Added reviewer into `Reviewers`
- [ ] I tagged(`@`) or used other form of notification of one person who I think can handle best review of this PR
- [ ] I have proved that PR has no general regressions of relevant features and processes required to release into production
- [ ] Any dependency updates made, was done according guides from relevant dependency
- Clicking all checkboxes 
- Adding detailed description of changes when it feels appropriate (for example when PR is big)

