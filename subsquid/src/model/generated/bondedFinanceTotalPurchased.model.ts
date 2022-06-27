import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"

@Entity_()
export class BondedFinanceTotalPurchased {
  constructor(props?: Partial<BondedFinanceTotalPurchased>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  /**
   * Total amount of purchased bonds
   */
  @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
  purchased!: bigint
}
