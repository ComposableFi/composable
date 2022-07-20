import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"

@Entity_()
export class BondedFinanceBondOffer {
  constructor(props?: Partial<BondedFinanceBondOffer>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  @Column_("text", {nullable: false})
  offerId!: string

  /**
   * Total amount of purchased bonds
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  totalPurchased!: bigint

  /**
   * Beneficiary account for the bond offer
   */
  @Column_("text", {nullable: false})
  beneficiary!: string
}
