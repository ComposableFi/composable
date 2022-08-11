import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {PicassoTransaction} from "./picassoTransaction.model"

@Entity_()
export class PicassoAccount {
  constructor(props?: Partial<PicassoAccount>) {
    Object.assign(this, props)
  }

  /**
   * Account address
   */
  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * Last transaction
   */
  @Column_("text", {nullable: false})
  transactionId!: string

  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  balance!: bigint

  @OneToMany_(() => PicassoTransaction, e => e.who)
  transactions!: PicassoTransaction[]
}
