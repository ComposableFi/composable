import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"
import {PabloAmount} from "./_pabloAmount"

@Entity_()
export class PabloLiquidityAdded {
    constructor(props?: Partial<PabloLiquidityAdded>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event | undefined | null

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @Column_("text", {nullable: false})
    blockId!: string

    @Column_("jsonb", {transformer: {to: obj => obj.map((val: any) => val.toJSON()), from: obj => obj == null ? undefined : marshal.fromList(obj, val => new PabloAmount(undefined, marshal.nonNull(val)))}, nullable: false})
    amounts!: (PabloAmount)[]

    @Column_("bool", {nullable: false})
    success!: boolean
}
