import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { HistoricalLockedValue, LockedSource } from "../../model";
import { getRange } from "./common";
import { PicassoTVL } from "./picassoOverviewStats";

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

      for (const assetId of assetIds) {
        for (const timestamp of timestamps) {
          const time = timestamp.toISOString();
          const row = await manager
            .getRepository(HistoricalLockedValue)
            .createQueryBuilder()
            .select(`'${time}'`, "date")
            .addSelect("asset_id", "assetId")
            .addSelect(`coalesce(tvl('${time}', '${lockedSource}', '${assetId}'), 0)`, "totalValueLocked")
            .getRawOne();

          lockedValues[time] = {
            ...(lockedValues[time] ?? {}),
            [assetId]: (lockedValues?.[time]?.[assetId] || 0n) + BigInt(row.totalValueLocked)
          };
        }
      }
    }

    return Object.keys(lockedValues).map(date => {
      const tvl: PicassoTVL[] = [];
      for (const assetId in lockedValues[date]) {
        tvl.push({ assetId, amount: lockedValues[date][assetId] });
      }

      return new TotalValueLocked({
        date,
        lockedValues: tvl
      });
    });
  }
}
