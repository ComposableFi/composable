import { ApiPromise } from "@polkadot/api";
import { Signer } from "@polkadot/api/types";
import { AnyAction, Dispatch } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

export class CrowdloanRewards {
  api: ApiPromise;
  dispatch: Dispatch<AnyAction>;

  constructor(api: ApiPromise, dispatch: Dispatch<AnyAction>) {
    this.api = api;
    this.dispatch = dispatch;
  }
  /**
   * Send association to picasso chain
   * @param proof Signed hash by contributor
   * @param rewardsAccount SS58 format string
   * @param contributorsAccount SS58 format string
   */
  public async associate(
    proof: string,
    rewardAccount: string,
    contributorAccount: string | undefined = undefined
  ) {
    const rewardsAccountID = this.api.createType("AccountId32", rewardAccount);

    const association = !!contributorAccount
      ? {
          RelayChain: [
            this.api.createType("AccountId32", contributorAccount),
            { Sr25519: proof },
          ],
        }
      : { Ethereum: proof };

    return this.api.tx.crowdloanRewards
      .associate(rewardsAccountID, association)
      .send();

    // executor.executeUnsigned(
    //   this.api.tx.crowdloanRewards
    //   .associate(rewardsAccountID, association),
    //   this.api,
    //   ()
    // )
  }
  /**
   * Claim Rewards
   * @param rewardsAccount SS58 format string
   */
  public async claim(rewardAccount: string, injectedSigner: Signer) {
    const methodResult = await this.api.tx.crowdloanRewards
      .claim()
      .signAndSend(rewardAccount, {
        signer: injectedSigner,
      });

    return methodResult.toHuman();
  }
  /**
   * Query association
   */
  public async association(userAccount: string) {
    let association: any = await this.api.query.crowdloanRewards.associations(
      userAccount
    );
    association = association.toHuman();

    return association;
  }

  /**
   * Query Rewards
   */
  public async queryRewards(userAccount: string, isRelayChain: boolean) {
    let param = isRelayChain
      ? this.api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          RelayChain: this.api.createType("AccountId32", userAccount),
        })
      : this.api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          Ethereum: userAccount,
        });

    let rewards: any = await this.api.query.crowdloanRewards.rewards(param);

    return rewards.toHuman();
  }

  public async queryAvailableToClaim(account: string) {
    let rpcRes = await this.api.rpc.crowdloanRewards.amountAvailableToClaimFor(
      account
    );

    let availableToClaim = rpcRes.toString();
    return availableToClaim;
  }

  public async queryInitialPayment(): Promise<string> {
    let initialPayment = await this.api.consts.crowdloanRewards.initialPayment;
    const inpBn = new BigNumber(initialPayment.toString());
    const decimals = new BigNumber(10).pow(9);
    const converted = inpBn.div(decimals).toString();
    return converted.toString();
  }
}
