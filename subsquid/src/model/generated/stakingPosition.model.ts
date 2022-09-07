import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"
import {StakingSource} from "./_stakingSource"

@Entity_()
export class StakingPosition {
  constructor(props?: Partial<StakingPosition>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * Unique identifier for the position on chain
   */
  @Column_("text", {nullable: false})
  positionId!: string

  @Column_("text", {nullable: false})
  owner!: string

  @Column_("text", {nullable: false})
  assetId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  amount!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  startTimestamp!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: true})
  endTimestamp!: bigint | undefined | null

  @Column_("varchar", {length: 14, nullable: false})
  source!: StakingSource
}
