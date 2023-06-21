import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, ManyToOne as ManyToOne_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {LockedSource} from "./_lockedSource"

@Index_(["assetId", "source"], {unique: true})
@Entity_()
export class CurrentLockedValue {
    constructor(props?: Partial<CurrentLockedValue>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("text", {nullable: false})
    assetId!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    amount!: bigint

    @Column_("varchar", {length: 16, nullable: false})
    source!: LockedSource
}
