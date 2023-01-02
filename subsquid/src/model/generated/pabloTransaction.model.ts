import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloTransaction {
    constructor(props?: Partial<PabloTransaction>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("text", {nullable: false})
    account!: string

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date
}
