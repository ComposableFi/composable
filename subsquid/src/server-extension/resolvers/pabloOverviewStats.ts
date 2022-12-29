import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { Account, HistoricalLockedValue, LockedSource } from "../../model";

@ObjectType()
class TVL {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;
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

  constructor(props: Partial<PabloOverviewStats>) {
    Object.assign(this, props);
  }
}

@Resolver(() => PabloOverviewStats)
export class PabloOverviewStatsResolver
  implements ResolverInterface<PabloOverviewStats>
{
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<TVL[]> {
    const manager = await this.tx();

    const lockedValue = await manager.find(HistoricalLockedValue, {
      select: ["amount", "assetId"],
      where: {
        source: LockedSource.Pablo,
      },
    });

    const totalValueLocked = lockedValue.reduce<Record<string, bigint>>(
      (acc, value) => {
        acc[value.assetId] = (acc[value.assetId] || 0n) + value.amount;
        return acc;
      },
      {}
    );

    const tvlList: TVL[] = [];

    Object.keys(totalValueLocked).forEach((assetId) => {
      const tvl = new TVL();
      tvl.assetId = assetId;
      tvl.amount = totalValueLocked[assetId];
      tvlList.push(tvl);
    });

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

    const averageLockMultiplier: { average_reward_multiplier: number }[] =
      await manager.getRepository(Account).query(
        `
        SELECT
            avg(reward_multiplier) as average_reward_multiplier
        FROM staking_position
        WHERE asset_id = '5'
      `
      );

    return Promise.resolve(
      averageLockMultiplier?.[0]?.average_reward_multiplier || 0
    );
  }

  @FieldResolver({ name: "averageLockTime", defaultValue: 0 })
  async averageLockTime(): Promise<number> {
    const manager = await this.tx();

    const averageDuration: { average_duration: number }[] = await manager
      .getRepository(Account)
      .query(
        `
        SELECT
            avg(duration) as average_duration
        FROM staking_position
        WHERE asset_id = '5'
      `
      );

    return Promise.resolve(averageDuration?.[0]?.average_duration || 0);
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
      })
    );
  }
}
