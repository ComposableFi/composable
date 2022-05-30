import { AssetId } from "@/defi/polkadot/types";
import { ConstantProductPool } from "../pools.types";

interface FormInput {
    baseAssetSelected: AssetId | "none";
    quoteAssetSelected: AssetId | "none";
    quoteAmount: string;
    baseAmount: string;    
}

interface PoolWithBalance extends ConstantProductPool {
    balance: {
        base: string;
        quote: string;
    }
}

export interface AddLiquiditySlice {
    addLiquidity: {
        form: FormInput;
        pool: PoolWithBalance,
        setFormField: (formFeildInput: Partial<FormInput>) => void;
        setPoolMetadata: (formFeildInput: Partial<PoolWithBalance>) => void;
    },
}
