import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
  Root,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { PicassoTransaction, Account, Activity } from "../../model";

@ObjectType()
export class OverviewCounter {
  @Field(() => Number, { nullable: false })
  transactionsCount!: number;

  @Field(() => Number, { nullable: false })
  accountHoldersCount!: number;

  @Field(() => Number, { nullable: false })
  activeUsersCount!: number;

  constructor(props: Partial<OverviewCounter>) {
    Object.assign(this, props);
  }
}

@Resolver((of) => OverviewCounter)
export class OverviewCountResolver
  implements ResolverInterface<OverviewCounter>
{
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "transactionsCount", defaultValue: 0 })
  async transactionsCount(@Root() overviewCounter: OverviewCounter) {
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

    return transactions?.[0]?.transactions_count || 0;
  }

  @FieldResolver({ name: "accountsHoldersCount", defaultValue: 0 })
  async accountHoldersCount(@Root() overviewCounter: OverviewCounter) {
    const manager = await this.tx();

    let accounts: any[] = await manager.getRepository(Account).query(
      `
        SELECT
          count(*) as accounts_count
        FROM account
        LIMIT 1
      `
    );

    return accounts?.[0]?.accounts_count || 0;
  }

  @FieldResolver({ name: "activeUsers", defaultValue: 0 })
  async activeUsersCount(@Root() overviewCounter: OverviewCounter) {
    const currentTimestamp = new Date().valueOf();
    const msPerDay = 24 * 60 * 60 * 1_000;
    const threshold = currentTimestamp - msPerDay;

    const manager = await this.tx();

    let activeUsers: any[] = await manager.getRepository(Activity).query(
      `
        SELECT
          count(distinct account_id) as active_users_count
        FROM activity
        WHERE timestamp > ${threshold}
      `
    );

    return activeUsers?.[0]?.active_users_count || 0;
  }

  @Query(() => OverviewCounter)
  async overviewCounter(): Promise<OverviewCounter> {
    // Default values
    return new OverviewCounter({
      transactionsCount: 0,
      accountHoldersCount: 0,
      activeUsersCount: 0,
    });
  }
}
