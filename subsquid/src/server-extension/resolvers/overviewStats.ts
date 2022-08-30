import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import {
  PicassoTransaction,
  Account,
  Activity,
  PicassoStakingPosition,
} from "../../model";

@ObjectType()
export class OverviewStats {
  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

  @Field(() => Number, { nullable: false })
  transactionsCount!: number;

  @Field(() => Number, { nullable: false })
  accountHoldersCount!: number;

  @Field(() => Number, { nullable: false })
  activeUsersCount!: number;

  constructor(props: Partial<OverviewStats>) {
    Object.assign(this, props);
  }
}

@Resolver(() => OverviewStats)
export class OverviewStatsResolver implements ResolverInterface<OverviewStats> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<bigint> {
    const now = new Date().valueOf();

    const manager = await this.tx();

    let picassoStakingPositions: { id: string; amount: number }[] =
      await manager.getRepository(PicassoStakingPosition).query(
        `
        SELECT
          id, amount
        FROM picasso_staking_position
        WHERE end_timestamp > ${now}
      `
      );

    const lockedValue = picassoStakingPositions.reduce(
      (acc, { amount }) => acc + BigInt(amount),
      0n
    );

    // TODO: add TVL from other sources

    return Promise.resolve(lockedValue);
  }

  @FieldResolver({ name: "transactionsCount", defaultValue: 0 })
  async transactionsCount(): Promise<number> {
    const manager = await this.tx();

    let transactions: { transactions_count: number }[] = await manager
      .getRepository(PicassoTransaction)
      .query(
        `
        SELECT
          count(*) as transactions_count
        FROM picasso_transaction
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

  @Query(() => OverviewStats)
  async overviewStats(): Promise<OverviewStats> {
    // Default values
    return Promise.resolve(
      new OverviewStats({
        totalValueLocked: 0n,
        transactionsCount: 0,
        accountHoldersCount: 0,
        activeUsersCount: 0,
      })
    );
  }
}
