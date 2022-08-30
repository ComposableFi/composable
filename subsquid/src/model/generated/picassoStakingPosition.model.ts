import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"

@Entity_()
export class PicassoStakingPosition {
  constructor(props?: Partial<PicassoStakingPosition>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  @Column_("text", {nullable: false})
  positionId!: string

  @Column_("text", {nullable: false})
  poolId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  amount!: bigint

  @Column_("text", {nullable: false})
  owner!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  startTimestamp!: bigint

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: true})
  endTimestamp!: bigint | undefined | null
}
