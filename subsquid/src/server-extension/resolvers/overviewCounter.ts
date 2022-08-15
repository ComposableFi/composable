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
import { PicassoTransaction, Account } from "../../model";

@ObjectType()
export class OverviewCounter {
  @Field(() => Number, { nullable: false })
  transactionsCount!: number;

  @Field(() => Number, { nullable: false })
  accountHoldersCount!: number;

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

  @Query(() => OverviewCounter)
  async overviewCounter(): Promise<OverviewCounter> {
    // Default values
    return new OverviewCounter({
      transactionsCount: 0,
      accountHoldersCount: 0,
    });
  }
}
