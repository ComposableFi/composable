import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"

@Entity_()
export class PabloLpToken {
    constructor(props?: Partial<PabloLpToken>) {
        Object.assign(this, props)
    }

    /**
     * LP token ID
     */
    @PrimaryColumn_()
    id!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalIssued!: bigint

    @Index_()
    @Column_("text", {nullable: false})
    poolId!: string

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    /**
     * Timestamp of the block in which this was last updated
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date
}
