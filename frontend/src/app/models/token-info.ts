export interface TokenInfo {
    contractAddress: string;
    name: string;
    symbol: string;
    decimals: number;
    totalSupply: string;
    owner?: string;
    creator?: string;
    isVerified: boolean;
    isRenounced: boolean;
    isActive: boolean;
    isV3: boolean;
    isScam: boolean;
    isRugPull: boolean;
    isDumpRisk: boolean;
    retryCount: number;
    previousContracts: number;
    liquidityPoolAddress?: string;
    liqudityPeriod: number;
    initialLiquidity: number;
    currentLiquidity: number;
    isLiquidyLocked: boolean;
    lockedLiquidity: number;
    isTaxModifiable: boolean;
    sellTax: number;
    buyTax: number;
    transferTax: number;
    score: number;
    holdersCount: number;
    data?: any; // Or a more specific type if you know the structure
    code?: string;
    abi?: string;
    error?: string;
    dateCreated: string; // Or Date if you're converting to JavaScript Date objects
    dateUpdated?: string; // Or Date
  }
