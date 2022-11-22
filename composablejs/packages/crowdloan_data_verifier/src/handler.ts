import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { Keyring } from "@polkadot/keyring";
import { AssertionError } from "assert";
// @ts-ignore
import * as intendedVestedTransfers from "@composable/crowdloan_data_verifier/transfers";

/**
 * Queries contributors on crowdloan pallet & fetches list of contributors from an url, as json file.
 *
 * ToDo: To get rid of all the @ts-ignore lines, one would need to define the json structure in TS.
 *
 * @param api Connected API Client
 * @param urlToContributorsFile String containing url to contributors list in json format.
 */
export async function verifyCrowdloanData(api: ApiPromise) {
  const rawPalletData = await api.query.crowdloanRewards.rewards.entries();
  const contributors = intendedVestedTransfers;
  if (!contributors) throw new AssertionError({ message:"Could not retrieve contributors list!" });
  if (rawPalletData.length == 0) throw new AssertionError({ message:"Chain wasn't populated yet! 0 contributors on chain." });

  const keyring = new Keyring();
  keyring.setSS58Format(49);
  for (const [contributor, contributorAmount] of rawPalletData) {
    let amountFromList: BN;
    let amountFromChain;

    // @ts-ignore
    if ("Ethereum" in contributor.toHuman()[0]) {
      // Handling ETH contributors

      // @ts-ignore
      if (!contributor.toHuman()[0]["Ethereum"]) continue;

      // Getting PICA amount for the according contributor & adjusting by 12 decimal places.
      // @ts-ignore
      const rawAmountFromList = contributors["rewardedPICAs"][contributor.toHuman()[0]["Ethereum"]];
      amountFromList = new BN(parseFloat(rawAmountFromList)).mul(new BN(10).pow(new BN(12)));

      // Getting PICA amount of contributor from chain list. (Already adjusted by 12 decimal places)
      // @ts-ignore
      amountFromChain = new BN(contributorAmount.toPrimitive()["total"]);
    } else {
      // Handling RelayChain contributors
      // @ts-ignore
      if (!contributor.toHuman()[0]["RelayChain"]) continue;
      // @ts-ignore
      const decodedValue = keyring.decodeAddress(contributor.toHuman()[0]["RelayChain"]);
      if (!decodedValue) continue;
      // @ts-ignore
      const contrib = contributors["rewardedPICAs"][keyring.encodeAddress(decodedValue, 2)];
      const rawAmountFromList = contrib;
      amountFromList = new BN(parseFloat(rawAmountFromList)).mul(new BN(10).pow(new BN(12)));

      // @ts-ignore
      amountFromChain = new BN(contributorAmount.toPrimitive()["total"]);
    }

    // If the amounts of a contributor do not align with the data on the chain,
    // we'll notify the user, and show him the wallet public key, intended amount & actual amount on chain.
    if (!amountFromList.eq(amountFromChain))
      console.warn(
        "\nDiscrepancy found!\n",
        contributor.toHuman(),
        " - Is:",
        amountFromChain.toString(),
        " - Should:",
        amountFromList.toString()
      );
  }

  // Now we're checking if the amount of contributors on chain equals the amount of contributors in our list.
  // And if this is not the case, we print out a warning.
  console.info("Finished crowdloan verification.");
  if (Object.keys(contributors["rewardedPICAs"]).length < rawPalletData.length) {
    console.warn("Warning: More contributors are on chain than the amount of contributors in provided list!");
    console.warn("Contributors on chain", rawPalletData.length);
    console.warn("Contributors in list", Object.keys(contributors["rewardedPICAs"]).length);
  } else if (Object.keys(contributors["rewardedPICAs"]).length > rawPalletData.length) {
    console.warn("Warning: Less contributors are on chain than the amount of contributors in provided list!");
    console.warn("Contributors on chain", rawPalletData.length);
    console.warn("Contributors in list", Object.keys(contributors["rewardedPICAs"]).length);
  }
}
