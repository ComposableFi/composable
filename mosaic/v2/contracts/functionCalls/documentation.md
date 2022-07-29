# todos

- rename msgReceiver to persona
- rename msgReceiverFactory to personaFactory
- todos from comments

# Miro

Miro board link: https://miro.com/app/board/o9J_lok9jCQ=/
Password: phase2calls

# Generic notes

- fees are taken on the destination layer to avoid any rebalancing (relayer will send the fee amount to the forwardCall
  method on MsgReceiver)
- user is responsible for having the necessary balances on the chains he wants to interact with
- SDK could abstract all steps

# User making a cross chain function call to layer B for the 1st time

step 1: create MsgReceiver on layer B by interacting with MsgReceiverFactory step 2: pre-fund your MsgReceiver with
funds (for fees or for any other operation you want to do in the future)
step 3: initiate from layer A a cross chain call to layer B

# User making a cross chain function call to layer B the 2nd time

step 1: initiate from layer A a cross chain call to layer B
