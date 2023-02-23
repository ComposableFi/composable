import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { MoreThan } from "typeorm";
import { IsString } from "class-validator";
import { PabloTransaction } from "../../model";
import { DAY_IN_MS } from "../../constants";
import { PoolAmount } from "./pabloDaily";

@ObjectType()
export class PabloDailyTxSwap {
  @Field(() => String, { nullable: false })
  quoteAssetId!: string;

  @Field(() => BigInt, { nullable: false })
  quoteAssetAmount!: bigint;

  @Field(() => String, { nullable: false })
  baseAssetId!: string;

  @Field(() => BigInt, { nullable: false })
  baseAssetAmount!: bigint;

  @Field(() => String, { nullable: false })
  spotPrice!: string;

  @Field(() => String, { nullable: false })
  feeAssetId!: string;

  @Field(() => BigInt, { nullable: false })
  feeAssetAmount!: bigint;

  constructor(props: PabloDailyTxSwap) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloDailyTx {
  @Field(() => String, { nullable: false })
  txHash!: string;

  @Field(() => [PoolAmount], { nullable: true })
  amounts?: PoolAmount[];

  @Field(() => String, { nullable: false })
  poolId!: string;

  @Field(() => Boolean, { nullable: false })
  success!: boolean;

  @Field(() => String, { nullable: true })
  failReason?: string;

  @Field(() => String, { nullable: false })
  txType!: string;

  @Field(() => Number, { nullable: false })
  timestamp!: number;

  @Field(() => PabloDailyTxSwap, { nullable: true })
  swap?: PabloDailyTxSwap;

  constructor(props: PabloDailyTx) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloDailyTransactions {
  @Field(() => [PabloDailyTx], { nullable: false })
  transactions!: PabloDailyTx[];

  constructor(props: Partial<PabloDailyTransactions>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloDailyTransactionsInput {
  @Field(() => String, { nullable: false })
  @IsString()
  address!: string;
}

@Resolver(() => PabloDailyTransactions)
export class PabloDailyTransactionsResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => PabloDailyTransactions)
  async pabloDailyTransactions(
    @Arg("params", { validate: true }) input: PabloDailyTransactionsInput
  ): Promise<PabloDailyTransactions> {
    const { address } = input;
    const manager = await this.tx();

    const dailyTransactions = await manager.find(PabloTransaction, {
      where: {
        account: address,
        timestamp: MoreThan(new Date(new Date().getTime() - DAY_IN_MS))
      },
      order: {
        timestamp: "DESC"
      },
      relations: {
        event: true,
        pool: true,
        swap: {
          fee: true
        },
        liquidityAdded: true,
        liquidityRemoved: true
      }
    });

    const transactions = dailyTransactions.map(tx => {
      return new PabloDailyTx({
        timestamp: tx.timestamp.getTime(),
        txHash: tx.event?.txHash || "",
        failReason: tx.failReason || undefined,
        txType: tx.txType,
        success: tx.success,
        poolId: tx.pool.id,
        amounts: tx.liquidityAdded?.amounts || tx.liquidityRemoved?.amounts || undefined,
        swap: tx.swap
          ? new PabloDailyTxSwap({
              baseAssetId: tx.swap.baseAssetId,
              baseAssetAmount: tx.swap.baseAssetAmount,
              quoteAssetId: tx.swap.quoteAssetId,
              quoteAssetAmount: tx.swap.quoteAssetAmount,
              feeAssetId: tx.swap.fee.assetId,
              feeAssetAmount: tx.swap.fee.fee,
              spotPrice: tx.swap.spotPrice
            })
          : undefined
      });
    });

    return Promise.resolve(
      new PabloDailyTransactions({
        transactions
      })
    );
  }
}
