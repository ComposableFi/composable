import { Field, FieldResolver, ObjectType, Query, Resolver, ResolverInterface } from "type-graphql";
import type { EntityManager } from "typeorm";
import { MoreThan } from "typeorm";
import { Account, PabloPoolAsset, PabloSwap } from "../../model";
import { DAY_IN_MS } from "./common";

@ObjectType()
class TVL {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;

  constructor(props: TVL) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloOverviewStats {
  @Field(() => [TVL], { nullable: false })
  totalValueLocked!: TVL[];

  @Field(() => BigInt, { nullable: false })
  totalXPicaMinted!: bigint;

  @Field(() => Number, { nullable: false })
  averageLockMultiplier!: number;

  @Field(() => Number, { nullable: false })
  averageLockTime!: number;

  @Field(() => [TVL], { nullable: false })
  dailyVolume!: TVL[];

  constructor(props: Partial<PabloOverviewStats>) {
    Object.assign(this, props);
  }
}

@Resolver(() => PabloOverviewStats)
export class PabloOverviewStatsResolver implements ResolverInterface<PabloOverviewStats> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<TVL[]> {
    const manager = await this.tx();

    const poolAssets = await manager.find(PabloPoolAsset, {
      select: ["assetId", "totalLiquidity"]
    });

    const totalValueLocked = poolAssets.reduce<Record<string, bigint>>((acc, curr) => {
      acc[curr.assetId] = (acc[curr.assetId] || 0n) + curr.totalLiquidity;
      return acc;
    }, {});

    const tvlList = Object.keys(totalValueLocked).map(
      assetId =>
        new TVL({
          assetId,
          amount: totalValueLocked[assetId]
        })
    );

    return Promise.resolve(tvlList);
  }

  @FieldResolver({ name: "totalXPicaMinted", defaultValue: 0 })
  async totalXPicaMinted(): Promise<bigint> {
    const manager = await this.tx();

    // TODO

    return Promise.resolve(0n);
  }

  @FieldResolver({ name: "averageLockMultiplier", defaultValue: 0 })
  async averageLockMultiplier(): Promise<number> {
    const manager = await this.tx();

    const averageLockMultiplier: { average_reward_multiplier: number }[] = await manager.getRepository(Account).query(
      `
        SELECT
            avg(reward_multiplier) as average_reward_multiplier
        FROM staking_position
        WHERE asset_id = '5'
      `
    );

    return Promise.resolve(averageLockMultiplier?.[0]?.average_reward_multiplier || 0);
  }

  @FieldResolver({ name: "averageLockTime", defaultValue: 0 })
  async averageLockTime(): Promise<number> {
    const manager = await this.tx();

    const averageDuration: { average_duration: number }[] = await manager.getRepository(Account).query(
      `
        SELECT
            avg(duration) as average_duration
        FROM staking_position
        WHERE asset_id = '5'
      `
    );

    return Promise.resolve(averageDuration?.[0]?.average_duration || 0);
  }

  @FieldResolver({ name: "dailyVolume" })
  async dailyVolume(): Promise<TVL[]> {
    const manager = await this.tx();

    const latestSwaps = await manager.getRepository(PabloSwap).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS))
      }
    });

    const volumes = latestSwaps.reduce<Record<string, bigint>>((acc, swap) => {
      acc[swap.baseAssetId] = (acc[swap.baseAssetId] || 0n) + swap.baseAssetAmount;
      acc[swap.quoteAssetId] = (acc[swap.quoteAssetId] || 0n) + swap.quoteAssetAmount;
      return acc;
    }, {});

    const tvlList = Object.keys(volumes).map(
      assetId =>
        new TVL({
          assetId,
          amount: volumes[assetId]
        })
    );

    return Promise.resolve(tvlList);
  }

  @Query(() => PabloOverviewStats)
  async pabloOverviewStats(): Promise<PabloOverviewStats> {
    // Default values
    return Promise.resolve(
      new PabloOverviewStats({
        totalValueLocked: [],
        totalXPicaMinted: 0n,
        averageLockMultiplier: 0,
        averageLockTime: 0,
        dailyVolume: []
      })
    );
  }
}
