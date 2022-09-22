import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToOne as OneToOne_, Index as Index_, JoinColumn as JoinColumn_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {Currency} from "./_currency"

@Entity_()
export class HistoricalLockedValue {
  constructor(props?: Partial<HistoricalLockedValue>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Index_({unique: true})
  @OneToOne_(() => Event, {nullable: false})
  @JoinColumn_()
  event!: Event

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  amount!: bigint

  @Column_("varchar", {length: 3, nullable: false})
  currency!: Currency

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint
}
