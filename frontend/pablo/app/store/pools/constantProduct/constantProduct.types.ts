export interface ConstantProductPool {
    poolId: number;
    owner: string;
    pair: {
      base: number;
      quote: number;
    }
    lpToken: string;
    fee: number;
    ownerFee: number;
}

export interface ConstantProductPoolsSlice {
    constantProductPools: { list: ConstantProductPool[]; }
    putConstantProductPools: (pools: ConstantProductPool[]) => void;
}