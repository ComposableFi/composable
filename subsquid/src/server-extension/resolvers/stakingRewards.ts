import { Field, FieldResolver, ObjectType, Query, Resolver, ResolverInterface } from "type-graphql";
import type { EntityManager } from "typeorm";
import { HistoricalLockedValue } from "../../model";

@ObjectType()
export class StakingRewardsStats {
  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

  constructor(props: Partial<StakingRewardsStats>) {
    Object.assign(this, props);
  }
}

@Resolver(() => StakingRewardsStats)
export class StakingRewardsStatsResolver implements ResolverInterface<StakingRewardsStats> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(): Promise<bigint> {
    const manager = await this.tx();

    let lockedValue: { amount: bigint }[] = await manager.getRepository(HistoricalLockedValue).query(
      `
        SELECT
          amount
        FROM historical_locked_value
        WHERE source = 'StakingRewards'
        ORDER BY timestamp DESC
        LIMIT 1
      `
    );

    if (!lockedValue?.[0]) {
      return Promise.resolve(0n);
    }

    return Promise.resolve(lockedValue[0].amount);
  }

  @Query(() => StakingRewardsStats)
  async stakingRewardsStats(): Promise<StakingRewardsStats> {
    // Default values
    return Promise.resolve(
      new StakingRewardsStats({
        totalValueLocked: 0n
      })
    );
  }
}
