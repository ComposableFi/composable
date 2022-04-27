"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.PabloSwappedEvent = exports.PabloPoolDeletedEvent = exports.PabloPoolCreatedEvent = exports.PabloLiquidityRemovedEvent = exports.PabloLiquidityAddedEvent = exports.BalancesTransferEvent = void 0;
const assert_1 = __importDefault(require("assert"));
const support_1 = require("./support");
class BalancesTransferEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'balances.Transfer');
    }
    /**
     * Transfer succeeded.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('balances.Transfer') === '0ffdf35c495114c2d42a8bf6c241483fd5334ca0198662e14480ad040f1e3a66';
    }
    /**
     * Transfer succeeded.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.BalancesTransferEvent = BalancesTransferEvent;
class PabloLiquidityAddedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.LiquidityAdded');
    }
    /**
     * Liquidity added into the pool `T::PoolId`.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.LiquidityAdded') === '312d582090ea3aa5c6ba6b929f4114d4a54ddca29cc066e4de5540c288ce5464';
    }
    /**
     * Liquidity added into the pool `T::PoolId`.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.PabloLiquidityAddedEvent = PabloLiquidityAddedEvent;
class PabloLiquidityRemovedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.LiquidityRemoved');
    }
    /**
     * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.LiquidityRemoved') === 'ef123c9326de7ce47d183c1b7d729db3c90f89a6bd64122aa03a48c169c6aa5b';
    }
    /**
     * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.PabloLiquidityRemovedEvent = PabloLiquidityRemovedEvent;
class PabloPoolCreatedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.PoolCreated');
    }
    /**
     * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.PoolCreated') === '76b660a348da63e9f657f2e6efbf072d8b02fe00cce4524df8e49986c270e996';
    }
    /**
     * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.PabloPoolCreatedEvent = PabloPoolCreatedEvent;
class PabloPoolDeletedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.PoolDeleted');
    }
    /**
     * The sale ended, the funds repatriated and the pool deleted.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.PoolDeleted') === '1b2177997ab30c1eecba237f26886dc4fce241682664c0c2ccd6fa478d585089';
    }
    /**
     * The sale ended, the funds repatriated and the pool deleted.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.PabloPoolDeletedEvent = PabloPoolDeletedEvent;
class PabloSwappedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.Swapped');
    }
    /**
     * Token exchange happened.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.Swapped') === 'cd4fbb8566d58553fc0cec0b6b7ee799d3f643b2953e2000db716e5919cb9214';
    }
    /**
     * Token exchange happened.
     */
    get asV2100() {
        (0, assert_1.default)(this.isV2100);
        return this.ctx._chain.decodeEvent(this.ctx.event);
    }
    get isLatest() {
        (0, support_1.deprecateLatest)();
        return this.isV2100;
    }
    get asLatest() {
        (0, support_1.deprecateLatest)();
        return this.asV2100;
    }
}
exports.PabloSwappedEvent = PabloSwappedEvent;
//# sourceMappingURL=events.js.map