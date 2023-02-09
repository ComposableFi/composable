import { Field, FieldResolver, ObjectType, Query, Resolver, ResolverInterface } from "type-graphql";
import type { EntityManager } from "typeorm";
import { DAY_IN_MS } from "./common";
import { Event, Account, Activity, HistoricalLockedValue } from "../../model";
import { getOrCreateAssetPrice } from "../../dbHelper";

@ObjectType()
export class PicassoTVL {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;

  @Field(() => Number, { nullable: true })
  price?: number;

  constructor(props: PicassoTVL) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PicassoOverviewStats {
  @Field(() => [PicassoTVL], { nullable: false })
  totalValueLocked!: PicassoTVL[];

  @Field(() => Number, { nullable: false })
  transactionsCount!: number;

  @Field(() => Number, { nullable: false })
  accountHoldersCount!: number;

  @Field(() => Number, { nullable: false })
  activeUsersCount!: number;

  constructor(props: Partial<PicassoOverviewStats>) {
    Object.assign(this, props);
  }
}

@Resolver(() => PicassoOverviewStats)
export class PicassoOverviewStatsResolver implements ResolverInterface<PicassoOverviewStats> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<PicassoTVL[]> {
    const manager = await this.tx();

    const lockedValues = await manager.getRepository(HistoricalLockedValue).find({
      select: ["amount", "assetId"]
    });

    const totalValueLocked = lockedValues.reduce<Record<string, bigint>>((acc, value) => {
      acc[value.assetId] = (acc[value.assetId] || 0n) + value.amount;
      return acc;
    }, {});

    const tvlList: PicassoTVL[] = [];

    for (const [assetId, amount] of Object.entries(totalValueLocked)) {
      const price = await getOrCreateAssetPrice(manager, assetId, new Date().getTime());
      const tvl = new PicassoTVL({
        assetId,
        amount,
        price
      });
      tvlList.push(tvl);
    }

    return Promise.resolve(tvlList);
  }

  @FieldResolver({ name: "transactionsCount", defaultValue: 0 })
  async transactionsCount(): Promise<number> {
    const manager = await this.tx();

    const count = await manager.getRepository(Event).count();

    return Promise.resolve(count || 0);
  }

  @FieldResolver({ name: "accountsHoldersCount", defaultValue: 0 })
  async accountHoldersCount(): Promise<number> {
    const manager = await this.tx();

    const count = await manager.getRepository(Account).count();

    return Promise.resolve(count);
  }

  @FieldResolver({ name: "activeUsers", defaultValue: 0 })
  async activeUsersCount(): Promise<number> {
    const manager = await this.tx();

    const { count } = await manager
      .getRepository(Activity)
      .createQueryBuilder()
      .select("COUNT(DISTINCT(account_id))", "count")
      .where(`timestamp > :timestamp`, { timestamp: new Date(new Date().getTime() - DAY_IN_MS) })
      .getRawOne();

    return Promise.resolve(count || 0);
  }

  @Query(() => PicassoOverviewStats)
  async overviewStats(): Promise<PicassoOverviewStats> {
    // Default values
    return Promise.resolve(
      new PicassoOverviewStats({
        totalValueLocked: [],
        transactionsCount: 0,
        accountHoldersCount: 0,
        activeUsersCount: 0
      })
    );
  }
}
