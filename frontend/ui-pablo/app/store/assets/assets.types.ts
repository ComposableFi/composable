export interface MockedAsset {
    name: string;
    decimals: number;
    symbol: string;
    icon: string;
    network: Record<string, string>
}

export interface AssetsSlice {
    supportedAssets: MockedAsset[];
    assetBalances: Record<string, Record<string, string>>,
    apollo: {
        [id: string]: string;
    }
    updateApolloPrice: (
        assetId: string,
        price: string
    ) => void;
    putAssetBalance: (
        networkId: string,
        assetId: string,
        balance: string
    ) => void;
}