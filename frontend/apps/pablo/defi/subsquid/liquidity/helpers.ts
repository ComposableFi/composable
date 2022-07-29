import { calcaulateProvidedLiquidity } from "@/defi/utils";
import { liquidityTransactionsByAddressAndPool } from "../pools/queries";
import BigNumber from "bignumber.js";

export async function fetchLiquidityProvided(
  accountId: string,
  poolId: string
): Promise<
  Record<
    string,
    {
      baseAmount: string;
      quoteAmount: string;
    }
  >
> {
  let liquidityRecord: Record<
    string,
    {
      baseAmount: string;
      quoteAmount: string;
    }
  > = {
    [poolId]: {
      baseAmount: "0",
      quoteAmount: "0",
    },
  };

  try {
    const response = await liquidityTransactionsByAddressAndPool(
      accountId,
      poolId
    );

    let { data, error } = response;

    if (error) throw new Error(error.message);
    let { pabloTransactions } = data;
    let liquidityProvided = calcaulateProvidedLiquidity(pabloTransactions);

    liquidityRecord[poolId].baseAmount = liquidityProvided.baseAmountProvided.toString();
    liquidityRecord[poolId].quoteAmount = liquidityProvided.quoteAmountProvided.toString();
  } catch (err) {
    console.error(err);
  }

  return liquidityRecord;
}
