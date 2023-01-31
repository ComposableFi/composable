import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { LessThan } from "typeorm";
import { HistoricalLockedValue, LockedSource, PabloPool } from "../../model";
import { getRange } from "./common";
import { PicassoTVL } from "./picassoOverviewStats";

@ObjectType()
export class PabloTVL {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => [PicassoTVL], { nullable: false })
  lockedValues!: PicassoTVL[];

  constructor(props: PabloTVL) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloTVLInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloTVLResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloTVL])
  async pabloTVL(@Arg("params", { validate: true }) input: PabloTVLInput): Promise<PabloTVL[]> {
    const { range, poolId } = input;

    const manager = await this.tx();

    const pool = await manager.getRepository(PabloPool).findOne({
      where: { id: poolId.toString() },
      order: { timestamp: "DESC" },
      relations: {
        poolAssets: true,
        poolAssetWeights: true
      }
    });

    if (!pool) {
      throw new Error(`Pool with id ${poolId} not found`);
    }

    const timestamps = getRange(range);
    // Map timestamp to tvl
    const lockedValues: Record<string, Record<string, bigint>> = timestamps.reduce((acc, timestamp) => {
      return {
        ...acc,
        [timestamp.toISOString()]: {}
      };
    }, {});

    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();

      const { quoteAssetId, poolAssets } = pool;
      const baseAssetId = poolAssets.map(({ assetId }) => assetId).find(assetId => assetId !== quoteAssetId)!;

      for (const assetId of [quoteAssetId, baseAssetId]) {
        const historicalLockedValue = await manager.getRepository(HistoricalLockedValue).findOne({
          where: {
            timestamp: LessThan(new Date(time)),
            source: LockedSource.Pablo,
            assetId,
            sourceEntityId: poolId
          },
          order: {
            timestamp: "DESC"
          }
        });

        lockedValues[time] = {
          ...(lockedValues[time] ?? {}),
          [assetId]: (lockedValues?.[time]?.[assetId] || 0n) + (historicalLockedValue?.accumulatedAmount || 0n)
        };
      }
    }

    return Object.keys(lockedValues).map(date => {
      const tvl: PicassoTVL[] = [];
      for (const assetId in lockedValues[date]) {
        tvl.push({ assetId, amount: lockedValues[date][assetId] });
      }

      return new PabloTVL({
        date,
        lockedValues: tvl
      });
    });
  }
}
