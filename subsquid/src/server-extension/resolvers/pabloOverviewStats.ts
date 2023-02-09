import { Field, FieldResolver, ObjectType, Query, Resolver, ResolverInterface } from "type-graphql";
import type { EntityManager } from "typeorm";
import { MoreThan } from "typeorm";
import { PabloPoolAsset, PabloSwap } from "../../model";
import { DAY_IN_MS } from "./common";
import { getOrCreateAssetPrice } from "../../dbHelper";

@ObjectType()
class TVL {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;

  @Field(() => Number, { nullable: true })
  price?: number;

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

    const tvlList: Array<TVL> = [];

    for (const assetId of Object.keys(totalValueLocked)) {
      const price = await getOrCreateAssetPrice(manager, assetId, new Date().getTime());
      tvlList.push(new TVL({ assetId, amount: totalValueLocked[assetId], price }));
    }

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

    // TODO: implement
    return Promise.resolve(0);
  }

  @FieldResolver({ name: "averageLockTime", defaultValue: 0 })
  async averageLockTime(): Promise<number> {
    const manager = await this.tx();

    // TODO: implement
    return Promise.resolve(0);
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
      acc[swap.quoteAssetId] = (acc[swap.quoteAssetId] || 0n) + swap.quoteAssetAmount;
      return acc;
    }, {});

    const tvlList: Array<TVL> = [];

    for (const assetId of Object.keys(volumes)) {
      const price = await getOrCreateAssetPrice(manager, assetId, new Date().getTime());
      tvlList.push(new TVL({ assetId, amount: volumes[assetId], price }));
    }

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
