import {
  Field,
  FieldResolver,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { Account, HistoricalLockedValue } from "../../model";

@ObjectType()
export class PabloOverviewStats {
  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

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
  async totalValueLocked(): Promise<bigint> {
    const manager = await this.tx();

    // TODO: add something like WHERE source = 'Pablo' once PR 1703 is merged
    let lockedValue: { amount: bigint }[] = await manager
      .getRepository(HistoricalLockedValue)
      .query(
        `
        SELECT
          amount
        FROM historical_locked_value
        WHERE source = 'Pablo'
        ORDER BY timestamp DESC
        LIMIT 1
      `
      );

    if (!lockedValue?.[0]) {
      return Promise.resolve(0n);
    }

    return Promise.resolve(lockedValue[0].amount);
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

    let averageLockMultiplier: { average_reward_multiplier: number }[] =
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

    let averageDuration: { average_duration: number }[] = await manager
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
        totalValueLocked: 0n,
        totalXPicaMinted: 0n,
        averageLockMultiplier: 0,
        averageLockTime: 0,
      })
    );
  }
}
