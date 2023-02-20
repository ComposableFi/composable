import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { IsDateString, IsString } from "class-validator";
import { getOrCreateHistoricalAssetPrice } from "../../dbHelper";

@ObjectType()
export class AssetInfo {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => Number, { nullable: true })
  price?: number;

  constructor(props: AssetInfo) {
    Object.assign(this, props);
  }
}

@InputType()
export class AssetsInput {
  @Field(() => String, { nullable: false })
  @IsString()
  assetId!: string;

  @Field(() => String, { nullable: true })
  @IsDateString()
  date?: string;
}

@Resolver()
export class AssetsResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => Number)
  async assetsPrices(@Arg("params", { validate: true }) input: AssetsInput): Promise<AssetInfo> {
    const { assetId, date } = input;

    const manager = await this.tx();

    const price = await getOrCreateHistoricalAssetPrice(manager, assetId, new Date(date || new Date()).getTime());

    return {
      assetId,
      price
    };
  }
}
