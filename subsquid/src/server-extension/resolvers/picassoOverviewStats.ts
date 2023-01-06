import { Field, FieldResolver, ObjectType, Query, Resolver, ResolverInterface } from "type-graphql";
import type { EntityManager } from "typeorm";
import { Event, Account, Activity, HistoricalLockedValue } from "../../model";

@ObjectType()
export class PicassoTVL {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;
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

    Object.keys(totalValueLocked).forEach(assetId => {
      const tvl = new PicassoTVL();
      tvl.assetId = assetId;
      tvl.amount = totalValueLocked[assetId];
      tvlList.push(tvl);
    });

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

    const accounts: { accounts_count: number }[] = await manager.getRepository(Account).query(
      `
        SELECT
          count(*) as accounts_count
        FROM account
        LIMIT 1
      `
    );

    return Promise.resolve(accounts?.[0]?.accounts_count || 0);
  }

  @FieldResolver({ name: "activeUsers", defaultValue: 0 })
  async activeUsersCount(): Promise<number> {
    const manager = await this.tx();

    const activeUsers: { active_users_count: number }[] = await manager.getRepository(Activity).query(
      `
        SELECT
          count(distinct account_id) as active_users_count
        FROM activity
        WHERE timestamp > current_timestamp - interval '1 day'
      `
    );

    return Promise.resolve(activeUsers?.[0]?.active_users_count || 0);
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
