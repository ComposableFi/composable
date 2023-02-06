import {
  Arg,
  Field,
  FieldResolver,
  InputType,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
  Root
} from "type-graphql";
import type { EntityManager } from "typeorm";
import BigNumber from "bignumber.js";
import { StakingPosition } from "../../model";

@ObjectType()
export class StakingRewardsStats {
  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

  @Field(() => BigInt, { nullable: false })
  averageLockDuration!: bigint;

  @Field(() => String, { nullable: false })
  poolId!: string;

  constructor(props: StakingRewardsStats) {
    Object.assign(this, props);
  }
}

@InputType()
export class StakingRewardsStatsInput {
  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver(() => StakingRewardsStats)
export class StakingRewardsStatsResolver implements ResolverInterface<StakingRewardsStats> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "totalValueLocked", defaultValue: 0 })
  async totalValueLocked(@Root() daily: StakingRewardsStats): Promise<bigint> {
    const { poolId } = daily;
    const manager = await this.tx();

    const stakingPositions = await manager.find(StakingPosition, {
      where: {
        assetId: poolId,
        removed: false
      }
    });

    const tvl = stakingPositions.reduce<bigint>((acc, position) => {
      return acc + position.amount;
    }, 0n);

    return Promise.resolve(tvl);
  }

  @FieldResolver({ name: "averageLockDuration", defaultValue: 0 })
  async averageLockDuration(@Root() daily: StakingRewardsStats): Promise<bigint> {
    const { poolId } = daily;
    const manager = await this.tx();

    const stakingPositions = await manager.find(StakingPosition, {
      where: {
        assetId: poolId,
        removed: false
      }
    });

    const tvl = stakingPositions.reduce<bigint>((acc, position) => {
      return acc + position.amount;
    }, 0n);

    const averageDuration = stakingPositions.reduce<BigNumber>((acc, position) => {
      const { duration, amount } = position;
      const share = BigNumber(amount.toString()).dividedBy(BigNumber(tvl.toString()));
      return acc.plus(share.multipliedBy(BigNumber(duration.toString())));
    }, BigNumber(0));

    return Promise.resolve(BigInt(averageDuration.toFixed(0)));
  }

  @Query(() => StakingRewardsStats)
  async stakingRewardsStats(
    @Arg("params", { validate: true }) input: StakingRewardsStatsInput
  ): Promise<StakingRewardsStats> {
    const { poolId } = input;
    // Default values
    return Promise.resolve(
      new StakingRewardsStats({
        poolId,
        totalValueLocked: 0n,
        averageLockDuration: 0n
      })
    );
  }
}
