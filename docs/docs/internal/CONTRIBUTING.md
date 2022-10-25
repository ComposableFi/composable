# CONTRIBUTING

We have a few guidelines and requirements to ensure correct workflows are followed while contributing to the composable project.

## First Things First
Before contributing, go through the documentation found in `~/docs/`.  Specifically:  
* [legacy/rust-setup.md](./legacy/rust-setup) ― A LEGACY walkthrough for installing and configuring Rust and other 
  tools in your development environment. **Use [Nix](../nix-overview.md) instead!**
* [proptest.md](./proptest) ― A guide to our approach and philosophy for automated testing  
* [benchmarking.md](./benchmarking) ― A guide for benchmarking pallet changes  

## Workflow
When contributing code changes, follow this general process to ensure the CI pipeline processes code and that your work is accessible via ClickUp.  
1) Create a git branch with a short name that is descriptive of the issue/feature you are working on  
2) Create a 'Draft' PR from your new branch. Include your ClickUp issue ID in either the title or description of the PR  
Example Title: `[CU-1u0b7bm] Added CONTRIBUTING.md`
3) Ensure your commits are signed and verified
4) Change the state of your PR to 'Open' once you are ready for review
5) Once all PR checks pass, a merge will be conducted

## Signing and Verification
All commits to the composable project ***must*** be signed and verified. GitHub provides an overview of commit signing that can be found [here](https://docs.github.com/en/authentication/managing-commit-signature-verification).  
Specifically, you will need to:  
1) You will need to have a [new](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key) or [existing](https://docs.github.com/en/authentication/managing-commit-signature-verification/checking-for-existing-gpg-keys) GPG key that is [associated to one of your valid GitHub emails](https://docs.github.com/en/authentication/managing-commit-signature-verification/associating-an-email-with-your-gpg-key)
2) [Add the GPG key to your GitHub account](https://docs.github.com/en/authentication/managing-commit-signature-verification/adding-a-new-gpg-key-to-your-github-account)
3) [Tell your local git client to use the GPG key](https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key)
4) Use the `-S` signing flag when you [commit](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)  
Alternatively, you can set `commit.gpgsign` to `true` so git will do this automatically
```bash
git config --global commit.gpgsign true
```

## Other Notes
* Keep the scope of PR changes small and easy to review
* Documentation throughout this repository will provide CLI commands in bash. You may need to use different commands/syntax if you use another shell
