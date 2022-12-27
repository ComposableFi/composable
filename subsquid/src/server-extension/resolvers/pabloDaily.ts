import {
  Arg,
  Field,
  FieldResolver,
  InputType,
  ObjectType,
  Query,
  Resolver,
  ResolverInterface,
  Root,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { MoreThan } from "typeorm";
import { PabloFee, PabloSwap, PabloTransaction } from "../../model";
import { DAY_IN_MS } from "./common";

@ObjectType()
export class PabloDaily {
  @Field(() => BigInt, { nullable: false })
  volume!: bigint;

  @Field(() => BigInt, { nullable: false })
  transactions!: bigint;

  @Field(() => BigInt, { nullable: false })
  fees!: bigint;

  @Field(() => String, { nullable: true, defaultValue: undefined })
  poolId?: string;

  constructor(props: Partial<PabloDaily>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloDailyInput {
  @Field(() => String, { defaultValue: undefined, nullable: true })
  poolId?: string;
}

@Resolver(() => PabloDaily)
export class PabloDailyResolver implements ResolverInterface<PabloDaily> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "volume", defaultValue: 0n })
  async volume(@Root() daily: PabloDaily): Promise<bigint> {
    const manager = await this.tx();

    const latestSwaps = await manager.getRepository(PabloSwap).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
        ...(daily.poolId && { pool: { id: daily.poolId } }),
      },
    });

    // TODO: get right volume
    const totalSwap = latestSwaps.reduce((acc, swap) => {
      return acc + 2n * swap.baseAssetAmount;
    }, 0n);

    return Promise.resolve(totalSwap);
  }

  @FieldResolver({ name: "transactions", defaultValue: 0n })
  async transactions(@Root() daily: PabloDaily): Promise<bigint> {
    const manager = await this.tx();

    const latestTransactions = await manager
      .getRepository(PabloTransaction)
      .find({
        where: {
          timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
          ...(daily.poolId && { pool: { id: daily.poolId } }),
        },
      });

    return Promise.resolve(BigInt(latestTransactions.length));
  }

  @FieldResolver({ name: "fees", defaultValue: 0n })
  async fees(@Root() daily: PabloDaily): Promise<bigint> {
    const manager = await this.tx();

    const latestFees = await manager.getRepository(PabloFee).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
        ...(daily.poolId && { pool: { id: daily.poolId } }),
      },
    });

    const totalFees = latestFees.reduce((acc, fee) => {
      return acc + fee.fee + fee.lpFee + fee.ownerFee + fee.protocolFee;
    }, 0n);

    return Promise.resolve(totalFees);
  }

  @Query(() => PabloDaily)
  async pabloDaily(
    @Arg("params", { validate: true }) input: PabloDailyInput
  ): Promise<PabloDaily> {
    // Default values
    return Promise.resolve(
      new PabloDaily({
        poolId: input.poolId,
        volume: 0n,
        transactions: 0n,
        fees: 0n,
      })
    );
  }
}
