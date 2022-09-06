# run-integration-tests.yml

## Description

This playbook does next:
* downloads `composable`, `basilisk` and `polkadot`
* installs and configures [certbot](https://certbot.eff.org/) to get a [Let’s Encrypt](https://letsencrypt.org/) certificate for your domain
* installs and configures [nginx](https://www.nginx.com/) to add a TLS termination using a [Let’s Encrypt](https://letsencrypt.org/) certificate
* runs local cluster of Polkadot with configured `composable` and `basilisk` parachains using [polkadot-launch](https://github.com/paritytech/polkadot-launch)
* runs [initialization script](https://github.com/ComposableFi/composable/tree/main/scripts/polkadot-launch/initialization) to add assets mappings in `composable` and `basilisk` parachains


## Usage

1. Create a VPS in your cloud
2. Create an A-type record in your DNS server referred to the external IP of your VPS created on 1st step
3. Add VPS in your inventory file
4. Run playbook: 

    ```bash
    ansible-playbook -i path_to_inventory .maintain/playbooks/run-integration-tests.yml -e "target=your_vps" -e "domain=domain_of_vps" -e "letsencrypt_contact_email=your_email@example.com" -e "github_user=your_github_account" -e "github_password=your_github_password_or_token"
    ```

## URLs

|           Node           |            URL           |
|:-------------------------|:------------------------:|
| Relay Chain #1           | wss://domain_of_vps:9901 |
| Composable's collator #1 | wss://domain_of_vps:9902 |


# dali-subsquid.yml

## Description
    This playbook deploys Subsquid for Dali.

## Usage

### Dev environment

    ```bash
    ansible-playbook -i inventory dev-dali-subsquid.yml -e "domain=dali-subsquid.composablefinance.ninja" -e "letsencrypt_contact_email=your-email@composable.finance"
    ```

### Prod environment

    ```bash
    ansible-playbook -i inventory prod-dali-subsquid.yml -e "domain=dali-subsquid.composable.finance"
    ```

# dali-stage.yml

## Description
    This playbook deploys Dali stage cluster.

## Usage

    ```bash
    ansible-playbook -i inventory dali-stage.yml -e "domain=dali-cluster-fe.composablefinance.ninja" -e "letsencrypt_contact_email=your-email@composable.finance" -e "composable_sandbox_tag=docker_tag"
    ```
