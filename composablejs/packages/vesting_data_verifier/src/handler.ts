import { ApiPromise } from "@polkadot/api";
import fetch from "node-fetch";
import papa from "papaparse";
import { AssertionError } from "assert";
import BN from "bn.js";


/**
 * Thanks to Saracha Tongkumpunt on StackOverflow for this snippet.
 * Source: https://stackoverflow.com/a/66149302
 */
const downloadCsv = async (url: string) => {
  try {
    const target = url;
    const res = await fetch(target, {
      method: "get",
      headers: {
        "content-type": "text/csv;charset=UTF-8"
      }
    });
    if (res.status === 200) {
      const data = await res.text();
      return data;

    } else {
      console.log(`Error code ${res.status}`);
    }
  } catch (err) {
    console.log(err);
  }
};

export function decodeVestingSchedule(vestingSchedule: any) {
  const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.blockNumberBased
      ? new BN(vestingSchedule.window.blockNumberBased.start)
      : new BN(vestingSchedule.window.momentBased.start),
    period: vestingSchedule.window.blockNumberBased
      ? new BN(vestingSchedule.window.blockNumberBased.period)
      : new BN(vestingSchedule.window.momentBased.period)
  };
  return {
    perPeriod: vestingSchedule.perPeriod,
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
    alreadyClaimed: vestingSchedule.alreadyClaimed,
    vestingScheduleId: new BN(vestingSchedule.vestingScheduleId)
  };
}

/**
 * Queries contributors on crowdloan pallet & fetches list of contributors from an url, as json file.
 *
 * ToDo: To get rid of all the @ts-ignore lines, one would need to define the json structure in TS.
 *
 * @param api Connected API Client
 * @param urlToContributorsFile String containing url to contributors list in json format.
 */
export async function verifyVestingPalletData(api: ApiPromise, urlToContributorsFile: string) {
  const expectedContributorsCSVRaw = await downloadCsv(urlToContributorsFile);
  if (!expectedContributorsCSVRaw) throw new AssertionError({ message: "Could not download contributors file!" });
  const expectedContributorsCSV = papa.parse(expectedContributorsCSVRaw);
  const expectedContributors = expectedContributorsCSV.data;

  // Skipping first entry since those are the table headers.
  for (let i = 1; i < expectedContributors.length; i++) {
    if ((<never[]>expectedContributors[i])[0] == "") continue; // Sometimes the CSV lib reads the last line as an empty entry.
    const expectedContributorPublicKey = api.createType("AccountId32", (<never[]>expectedContributors[i])[0]);

    const onChainContributorData = await api.query.vesting.vestingSchedules(expectedContributorPublicKey, 1);
    const _schedules = onChainContributorData.toJSON();
    const schedules = Object.values(_schedules as any).map(i => decodeVestingSchedule(i));

    // @ts-ignore
    const expectedFullTransferAmount = new BN((expectedContributors[i][1]).replaceAll(",", ""));
    if (schedules.toString() == "") {
      console.warn("\nWARNING: Contributor",
        expectedContributorPublicKey.toString(),
        "has NO vested transfers entries!\nExpected:", expectedFullTransferAmount.toString() + "\n"
      );
      continue;
    }
    for (const schedule of schedules) {
      const fullVestedTransferAmount = new BN(schedule.perPeriod * (schedule.periodCount));

      if (!fullVestedTransferAmount.eq(expectedFullTransferAmount)) {
        console.warn("\nDiscrepancy found!", expectedContributorPublicKey.toString(),
          "\nExpected:", expectedFullTransferAmount.toString(), " - Is:", fullVestedTransferAmount.toString());
      }
    }
  }
}
