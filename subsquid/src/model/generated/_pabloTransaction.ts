import {PabloTransactionCreatePool} from "./_pabloTransactionCreatePool"
import {PabloTransactionAddLiquidity} from "./_pabloTransactionAddLiquidity"
import {PabloTransactionRemoveLiquidity} from "./_pabloTransactionRemoveLiquidity"
import {PabloTransactionSwap} from "./_pabloTransactionSwap"

export type PabloTransaction = PabloTransactionCreatePool | PabloTransactionAddLiquidity | PabloTransactionRemoveLiquidity | PabloTransactionSwap

export function fromJsonPabloTransaction(json: any): PabloTransaction {
    switch(json?.isTypeOf) {
        case 'PabloTransactionCreatePool': return new PabloTransactionCreatePool(undefined, json)
        case 'PabloTransactionAddLiquidity': return new PabloTransactionAddLiquidity(undefined, json)
        case 'PabloTransactionRemoveLiquidity': return new PabloTransactionRemoveLiquidity(undefined, json)
        case 'PabloTransactionSwap': return new PabloTransactionSwap(undefined, json)
        default: throw new TypeError('Unknown json object passed as PabloTransaction')
    }
}
