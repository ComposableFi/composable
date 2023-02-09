import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { LessThan } from "typeorm";
import { HistoricalLockedValue, LockedSource } from "../../model";
import { getRange } from "./common";
import { PicassoTVL } from "./picassoOverviewStats";
import { getOrCreateAssetPrice } from "../../dbHelper";

@ObjectType()
export class TotalValueLocked {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => [PicassoTVL], { nullable: false })
  lockedValues!: PicassoTVL[];

  constructor(props: Partial<TotalValueLocked>) {
    Object.assign(this, props);
  }
}

@InputType()
export class TotalValueLockedInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: true })
  source?: LockedSource;
}

@Resolver()
export class TotalValueLockedResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalValueLocked])
  async totalValueLocked(@Arg("params", { validate: true }) input: TotalValueLockedInput): Promise<TotalValueLocked[]> {
    const { range, source } = input;

    const manager = await this.tx();

    const sources = source ? [source] : Object.values(LockedSource);
    const timestamps = getRange(range);

    // Map timestamp to {assetId -> tvl}
    const lockedValues: Record<string, Record<string, bigint>> = timestamps.reduce((acc, timestamp) => {
      return {
        ...acc,
        [timestamp.toISOString()]: {}
      };
    }, {});

    for (const lockedSource of sources) {
      const assetIds: string[] = (
        await manager
          .getRepository(HistoricalLockedValue)
          .createQueryBuilder("value")
          .select("value.assetId", "assetId")
          .where("value.source = :source", { source: lockedSource })
          .groupBy("value.assetId")
          .getRawMany()
      ).map(row => row.assetId);

      const entityIds: string[] = (
        await manager
          .getRepository(HistoricalLockedValue)
          .createQueryBuilder("value")
          .select("value.sourceEntityId", "sourceEntityId")
          .where("value.source = :source", { source: lockedSource })
          .groupBy("value.sourceEntityId")
          .getRawMany()
      ).map(row => row.sourceEntityId);

      for (const assetId of assetIds) {
        for (const entityId of entityIds) {
          for (const timestamp of timestamps) {
            const time = timestamp.toISOString();

            const historicalLockedValue = await manager.getRepository(HistoricalLockedValue).findOne({
              where: {
                timestamp: LessThan(new Date(time)),
                source: lockedSource,
                assetId,
                sourceEntityId: entityId
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
      }
    }

    const totalValueLocked: TotalValueLocked[] = [];

    for (const time of Object.keys(lockedValues)) {
      const tvl: PicassoTVL[] = [];
      for (const assetId of Object.keys(lockedValues[time])) {
        const price = await getOrCreateAssetPrice(manager, assetId, new Date(time).getTime());
        tvl.push({
          assetId,
          amount: lockedValues[time][assetId],
          price
        });
      }
      totalValueLocked.push({
        date: time,
        lockedValues: tvl
      });
    }

    return totalValueLocked;
  }
}
