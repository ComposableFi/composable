# How to use the VM

## Components

### VMInput

---------
Component that has all the initialization related code as props.
Has its own ID which will be referenced by other components displaying the vm state.

### SingleValue

---------
Component that displays anything from the VmState. Typically returns a single value wrapped in a component.
Headless by default.


## Misc

- imports are relative due to spending too much time trying to make tsconfig import properly while keeping the docusaurus weird `@site` import work