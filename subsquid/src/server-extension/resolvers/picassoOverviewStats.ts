import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { Event, Account, Activity, HistoricalLockedValue } from "../../model";

@ObjectType()
export class PicassoOverviewStats {
  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

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
export class PicassoOverviewStatsResolver
  implements ResolverInterface<PicassoOverviewStats>
{
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<bigint> {
    const manager = await this.tx();

    // TODO: add something like WHERE source = 'All' once PR 1703 is merged
    let lockedValue: { amount: bigint }[] = await manager
      .getRepository(HistoricalLockedValue)
      .query(
        `
        SELECT
          amount
        FROM historical_locked_value
        ORDER BY timestamp DESC
        LIMIT 1
      `
      );

    if (!lockedValue?.[0]) {
      return Promise.resolve(0n);
    }

    return Promise.resolve(lockedValue[0].amount);
  }

  @FieldResolver({ name: "transactionsCount", defaultValue: 0 })
  async transactionsCount(): Promise<number> {
    const manager = await this.tx();

    let transactions: { transactions_count: number }[] = await manager
      .getRepository(Event)
      .query(
        `
        SELECT
          count(*) as transactions_count
        FROM event
        LIMIT 1
      `
      );

    return Promise.resolve(transactions?.[0]?.transactions_count || 0);
  }

  @FieldResolver({ name: "accountsHoldersCount", defaultValue: 0 })
  async accountHoldersCount(): Promise<number> {
    const manager = await this.tx();

    let accounts: { accounts_count: number }[] = await manager
      .getRepository(Account)
      .query(
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
    const currentTimestamp = new Date().valueOf();
    const msPerDay = 24 * 60 * 60 * 1_000;
    const threshold = currentTimestamp - msPerDay;

    const manager = await this.tx();

    const activeUsers: { active_users_count: number }[] = await manager
      .getRepository(Activity)
      .query(
        `
        SELECT
          count(distinct account_id) as active_users_count
        FROM activity
        WHERE timestamp > ${threshold}
      `
      );

    return Promise.resolve(activeUsers?.[0]?.active_users_count || 0);
  }

  @Query(() => PicassoOverviewStats)
  async overviewStats(): Promise<PicassoOverviewStats> {
    // Default values
    return Promise.resolve(
      new PicassoOverviewStats({
        totalValueLocked: 0n,
        transactionsCount: 0,
        accountHoldersCount: 0,
        activeUsersCount: 0,
      })
    );
  }
}
