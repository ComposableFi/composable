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
import { MoreThan } from "typeorm";
import { PabloFee, PabloPool, PabloSwap, PabloTransaction } from "../../model";
import { DAY_IN_MS } from "./common";
import { getOrCreateAssetPrice } from "../../dbHelper";

@ObjectType()
export class PoolAmount {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;

  @Field(() => Number, { nullable: true })
  price?: number;

  constructor(props: PoolAmount) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloDaily {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => [PoolAmount], { nullable: false })
  volume!: PoolAmount[];

  @Field(() => BigInt, { nullable: false })
  transactions!: bigint;

  @Field(() => [PoolAmount], { nullable: false })
  fees!: PoolAmount[];

  @Field(() => String, { nullable: false })
  poolId!: string;

  constructor(props: Partial<PabloDaily>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloDailyInput {
  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver(() => PabloDaily)
export class PabloDailyResolver implements ResolverInterface<PabloDaily> {
  constructor(private tx: () => Promise<EntityManager>) {}

  @FieldResolver({ name: "volume", defaultValue: 0n })
  async volume(@Root() daily: PabloDaily): Promise<PoolAmount[]> {
    const manager = await this.tx();

    const latestSwaps = await manager.getRepository(PabloSwap).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
        pool: {
          id: daily.poolId
        }
      }
    });

    const swapsMap = latestSwaps.reduce<Record<string, bigint>>((acc, swap) => {
      acc[swap.quoteAssetId] = (acc[swap.quoteAssetId] || 0n) + swap.quoteAssetAmount;
      return acc;
    }, {});

    const totalVolumes: Array<PoolAmount> = [];

    for (const assetId of Object.keys(swapsMap)) {
      const price = await getOrCreateAssetPrice(manager, assetId, new Date().getTime());
      totalVolumes.push(
        new PoolAmount({
          assetId,
          amount: swapsMap[assetId],
          price
        })
      );
    }

    return Promise.resolve(totalVolumes);
  }

  @FieldResolver({ name: "transactions", defaultValue: 0n })
  async transactions(@Root() daily: PabloDaily): Promise<bigint> {
    const manager = await this.tx();

    const latestTransactions = await manager.getRepository(PabloTransaction).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
        pool: {
          id: daily.poolId
        }
      }
    });

    return Promise.resolve(BigInt(latestTransactions.length));
  }

  @FieldResolver({ name: "fees", defaultValue: 0n })
  async fees(@Root() daily: PabloDaily): Promise<PoolAmount[]> {
    const manager = await this.tx();

    const latestFees = await manager.getRepository(PabloFee).find({
      where: {
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS)),
        ...(daily.poolId && { pool: { id: daily.poolId } })
      }
    });

    const feesMap = latestFees.reduce<Record<string, bigint>>((acc, fee) => {
      acc[fee.assetId] = (acc[fee.assetId] || 0n) + fee.fee;
      return acc;
    }, {});

    const totalFees: Array<PoolAmount> = [];

    for (const assetId of Object.keys(feesMap)) {
      const price = await getOrCreateAssetPrice(manager, assetId, new Date().getTime());
      totalFees.push(
        new PoolAmount({
          assetId,
          amount: feesMap[assetId],
          price
        })
      );
    }

    return Promise.resolve(totalFees);
  }

  @FieldResolver({ name: "assetId" })
  async assetId(@Root() daily: PabloDaily): Promise<string> {
    const manager = await this.tx();

    const pool = await manager.getRepository(PabloPool).findOne({
      where: {
        id: daily.poolId
      }
    });

    if (!pool) {
      throw new Error(`Pool ${daily.poolId} not found`);
    }

    return Promise.resolve(pool.quoteAssetId);
  }

  @Query(() => PabloDaily)
  async pabloDaily(@Arg("params", { validate: true }) input: PabloDailyInput): Promise<PabloDaily> {
    // Default values
    return Promise.resolve(
      new PabloDaily({
        poolId: input.poolId,
        assetId: "",
        volume: [],
        transactions: 0n,
        fees: []
      })
    );
  }
}
