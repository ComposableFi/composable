import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { LessThan } from "typeorm";
import { IsEnum, IsString } from "class-validator";
import { HistoricalLockedValue, LockedSource, PabloPool } from "../../model";
import { getRange } from "./common";
import { PicassoTVL } from "./picassoOverviewStats";
import { getOrCreateHistoricalAssetPrice } from "../../dbHelper";

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
  @IsEnum(["day", "week", "month", "year"])
  range!: string;

  @Field(() => String, { nullable: true })
  @IsString()
  poolId?: string;
}

@Resolver()
export class PabloTVLResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloTVL])
  async pabloTVL(@Arg("params", { validate: true }) input: PabloTVLInput): Promise<PabloTVL[]> {
    const { range, poolId } = input;

    const manager = await this.tx();

    const pools = await manager.getRepository(PabloPool).find({
      ...(poolId ? { where: { id: poolId.toString() } } : {}),
      order: { timestamp: "DESC" },
      relations: {
        poolAssets: true,
        poolAssetWeights: true
      }
    });

    if (!pools.length) {
      throw new Error(`Pool/s not found`);
    }

    const timestamps = getRange(range);
    // Map timestamp to tvl
    const lockedValues: Record<string, Record<string, bigint>> = timestamps.reduce((acc, timestamp) => {
      return {
        ...acc,
        [timestamp.toISOString()]: {}
      };
    }, {});

    for (const pool of pools) {
      const { quoteAssetId, poolAssets } = pool;
      const baseAssetId = poolAssets.map(({ assetId }) => assetId).find(assetId => assetId !== quoteAssetId)!;

      for (const timestamp of timestamps) {
        const time = timestamp.toISOString();

        for (const assetId of [quoteAssetId, baseAssetId]) {
          const historicalLockedValue = await manager.getRepository(HistoricalLockedValue).findOne({
            where: {
              timestamp: LessThan(new Date(time)),
              source: LockedSource.Pablo,
              assetId,
              sourceEntityId: pool.id
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

    const pabloTVL: Array<PabloTVL> = [];

    for (const date of Object.keys(lockedValues)) {
      const tvl: PicassoTVL[] = [];
      for (const assetId of Object.keys(lockedValues[date])) {
        if (lockedValues[date][assetId]) {
          const price = await getOrCreateHistoricalAssetPrice(manager, assetId, new Date(date).getTime());
          tvl.push({ assetId, amount: lockedValues[date][assetId], price });
        }
      }

      pabloTVL.push(new PabloTVL({ date, lockedValues: tvl }));
    }

    return pabloTVL;
  }
}
