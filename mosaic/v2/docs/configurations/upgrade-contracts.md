# How to deploy upgradable contract

`TransparentProxy` pattern is used for all the upgradable contracts

## Deploy

In order to deploy the contract, admin need to run pre-configured scripts

Each deployment script is stored in `deploy` folder

E.g:

```shell
NODE_ENV=<network> yarn deploy:mosaic_vault <network>
```

The contract is deployed and initialized. Same for both proxy and the implementation.

Deployment artifacts (proxy admin, contract implementation, contract proxy)
are stored in `deployment/<network>/<contract>` folder.

## Upgrade

After performing some changes to the contract, same script can be used to upgrade the contract

Only the implementation will be deployed

E.g:

```shell
NODE_ENV=<network> yarn deploy:mosaic_vault <network>
```

Contract will be upgraded only if some changes have been made to the contract.

If no changes have been made, the script **skip** de deployment

The script is fully automatic. It deploys, initialize and change the proxy to point to the new implementation.

> :warning: The library used for the deployment doesn't check if the upgrade is safe. More
> [here](https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies#unstructured-storage-proxies)
> and
> [here](https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies#storage-collisions-between-implementation-versions)
