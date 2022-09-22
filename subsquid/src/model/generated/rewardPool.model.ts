import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_} from "typeorm"

@Entity_()
export class RewardPool {
  constructor(props?: Partial<RewardPool>) {
    Object.assign(this, props)
  }

  @PrimaryColumn_()
  id!: string

  @Index_()
  @Column_("text", {nullable: false})
  eventId!: string

  @Index_()
  @Column_("text", {nullable: false})
  poolId!: string
}
