import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {Currency} from "./_currency"
import {LockedSource} from "./_lockedSource"

@Entity_()
export class HistoricalLockedValue {
  constructor(props?: Partial<HistoricalLockedValue>) {
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

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint

  @Column_("varchar", {length: 16, nullable: false})
  source!: LockedSource
}
