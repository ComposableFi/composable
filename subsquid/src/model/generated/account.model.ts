import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"

@Entity_()
export class Account {
    constructor(props?: Partial<Account>) {
        Object.assign(this, props)
    }

    /**
     * Account address
     */
    @PrimaryColumn_()
    id!: string

    /**
     * Last event involving account
     */
    @Column_("text", {nullable: false})
    eventId!: string

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string
}
