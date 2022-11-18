import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { getAmountWithoutDecimals } from "../../utils";
import {
  Event,
  Account,
  Activity,
  CurrentLockedValue,
  LockedSource,
  Asset,
} from "../../model";
import { chain } from "../../config";

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
    const wsProvider = new WsProvider(chain());
    const api = await ApiPromise.create({ provider: wsProvider });

    const manager = await this.tx();

    const lockedValue = await manager.getRepository(CurrentLockedValue).find({
      where: {
        source: LockedSource.All,
      },
    });

    const assetsInfo = lockedValue.reduce<
      Record<string, { price: bigint; decimals: number }>
    >((acc, value) => {
      acc[value.assetId] = {
        price: 0n,
        decimals: 12,
      };
      return acc;
    }, {});

    for (const assetId of Object.keys(assetsInfo)) {
      try {
        const oraclePrice = await api.query.oracle.prices(assetId);
        const asset = await manager.getRepository(Asset).findOne({
          where: {
            id: assetId,
          },
        });
        if (oraclePrice?.price) {
          assetsInfo[assetId] = {
            price: BigInt(oraclePrice?.price?.toString() || 0n),
            decimals: asset?.decimals || 12,
          };
        }
      } catch (err) {
        console.log("Error:", err);
      }
    }

    const totalLockedValue = lockedValue.reduce((acc, value) => {
      const { assetId } = value;
      const { price, decimals } = assetsInfo[assetId];

      if (!price) {
        return acc;
      }

      const lockedValue = getAmountWithoutDecimals(
        value.amount,
        decimals
      ).toString();

      return acc + price * BigInt(lockedValue);
    }, 0n);

    return Promise.resolve(totalLockedValue);
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
