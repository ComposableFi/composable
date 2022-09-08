import {
  Column as Column_,
  Entity as Entity_,
  OneToMany as OneToMany_,
  PrimaryColumn as PrimaryColumn_,
} from "typeorm";
import * as marshal from "./marshal";
import { HistoricalBalance } from "./historicalBalance.model";

@Entity_()
export class Account {
  constructor(props?: Partial<Account>) {
    Object.assign(this, props);
  }

  /**
   * Account address
   */
  @PrimaryColumn_()
  id!: string;

  @Column_("numeric", {
    transformer: marshal.bigintTransformer,
    nullable: false,
  })
  balance!: bigint;

  @OneToMany_(() => HistoricalBalance, (e) => e.account)
  historicalBalances!: HistoricalBalance[];
}
