import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {Currency} from "./_currency"

@Entity_()
export class HistoricalVolume {
  constructor(props?: Partial<HistoricalVolume>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Index_()
  @ManyToOne_(() => Event, {nullable: true})
  event!: Event

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  amount!: bigint

  @Column_("varchar", {length: 3, nullable: false})
  currency!: Currency

  @Index_()
  @Column_("timestamp with time zone", {nullable: false})
  timestamp!: Date

  @Column_("text", {nullable: false})
  assetId!: string
}
