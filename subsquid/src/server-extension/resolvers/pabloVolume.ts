import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { HistoricalVolume, PabloPool } from "../../model";
import { getRange } from "./common";

@ObjectType()
export class PabloVolume {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => BigInt, { nullable: false })
  volume!: bigint;

  constructor(props: Partial<PabloVolume>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloVolumeInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;

  @Field(() => String, { nullable: false })
  assetId!: string;
}

@Resolver()
export class PabloVolumeResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloVolume])
  async pabloVolume(@Arg("params", { validate: true }) input: PabloVolumeInput): Promise<PabloVolume[]> {
    const { range, poolId, assetId } = input;

    const manager = await this.tx();

    const pool = await manager.getRepository(PabloPool).findOne({
      where: { id: poolId.toString() },
      relations: {
        poolAssets: true,
        poolAssetWeights: true
      }
    });

    if (!pool) {
      throw new Error(`Pool with id ${poolId} not found`);
    }

    const timestamps = getRange(range);
    // Map timestamp to volume
    const volumes: Record<string, bigint> = {};

    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();

      const volume = await manager
        .getRepository(HistoricalVolume)
        .createQueryBuilder()
        .select("COALESCE(accumulated_amount, 0)", "amount")
        .where("timestamp < :time", { time })
        .andWhere("pool_id = :poolId", { poolId })
        .andWhere("asset_id = :assetId", { assetId })
        .orderBy("timestamp", "DESC")
        .limit(1)
        .getRawOne();

      volumes[time] = BigInt(volume?.amount || 0);
    }

    return Object.keys(volumes).map(date => {
      return new PabloVolume({
        date,
        volume: volumes[date]
      });
    });
  }
}
