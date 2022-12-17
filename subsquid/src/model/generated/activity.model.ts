import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {Event} from "./event.model"

@Entity_()
export class Activity {
    constructor(props?: Partial<Activity>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    /**
     * ID of the event associated with this activity
     */
    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event

    /**
     * ID of the active account
     */
    @Index_()
    @Column_("text", {nullable: false})
    accountId!: string

    /**
     * Timestamp of the block in which this activity occurred
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date
}
