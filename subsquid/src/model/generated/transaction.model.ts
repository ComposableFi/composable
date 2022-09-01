import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {TransactionType} from "./_transactionType"
import {PabloTransaction} from "./pabloTransaction.model"

@Entity_()
export class Transaction {
  constructor(props?: Partial<Transaction>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * ID of the event that was used to derive this transaction
   */
  @Column_("text", {nullable: false})
  eventId!: string

  /**
   * ID of account that executed transaction
   */
  @Column_("text", {nullable: false})
  accountId!: string

  /**
   * Type of transaction
   */
  @Column_("varchar", {length: 43, nullable: false})
  transactionType!: TransactionType

  /**
   * Block in which transaction was registered
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  blockNumber!: bigint

  /**
   * Timestamp of the block in which this transaction was registered
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  timestamp!: bigint

  /**
   * If this transaction came from Pablo, it will have extra information
   */
  @Index_()
  @ManyToOne_(() => PabloTransaction, {nullable: true})
  pabloTransaction!: PabloTransaction | undefined | null
}
