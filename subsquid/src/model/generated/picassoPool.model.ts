import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"

@Entity_()
export class PicassoPool {
  constructor(props?: Partial<PicassoPool>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Column_("text", {nullable: false})
  eventId!: string

  @Column_("text", {nullable: false})
  poolId!: string

  @Column_("text", {nullable: false})
  assetId!: string
}
