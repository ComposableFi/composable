"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.PabloPoolCreatedEvent = exports.BalancesTransferEvent = void 0;
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
class PabloPoolCreatedEvent {
    constructor(ctx) {
        this.ctx = ctx;
        (0, assert_1.default)(this.ctx.event.name === 'pablo.PoolCreated');
    }
    /**
     * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
     */
    get isV2100() {
        return this.ctx._chain.getEventHash('pablo.PoolCreated') === '9d2b9ca9cc54280587b78a037ab5d28ac846875ec675325c76892e5e5cdfa3fe';
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
//# sourceMappingURL=events.js.map