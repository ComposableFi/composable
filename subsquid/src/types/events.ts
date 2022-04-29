import assert from 'assert'
import {EventContext, Result, deprecateLatest} from './support'
import * as v2100 from './v2100'

export class BalancesTransferEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'balances.Transfer')
  }

  /**
   * Transfer succeeded.
   */
  get isV2100(): boolean {
    return this.ctx._chain.getEventHash('balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66'
  }

  /**
   * Transfer succeeded.
   */
  get asV2100(): {from: v2100.AccountId32, to: v2100.AccountId32, amount: bigint} {
    assert(this.isV2100)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2100
  }

  get asLatest(): {from: v2100.AccountId32, to: v2100.AccountId32, amount: bigint} {
    deprecateLatest()
    return this.asV2100
  }
}

export class PabloPoolCreatedEvent {
  constructor(private ctx: EventContext) {
    assert(this.ctx.event.name === 'pablo.PoolCreated')
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get isV2100(): boolean {
    return this.ctx._chain.getEventHash('pablo.PoolCreated') === '9d2b9ca9cc54280587b78a037ab5d28ac846875ec675325c76892e5e5cdfa3fe'
  }

  /**
   * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
   */
  get asV2100(): {poolId: bigint, owner: v2100.AccountId32} {
    assert(this.isV2100)
    return this.ctx._chain.decodeEvent(this.ctx.event)
  }

  get isLatest(): boolean {
    deprecateLatest()
    return this.isV2100
  }

  get asLatest(): {poolId: bigint, owner: v2100.AccountId32} {
    deprecateLatest()
    return this.asV2100
  }
}
