export interface VaultState {
    totalAssets: number;
    liabilities: number;
    lastUpdate: number;
}

export interface UserProfile {
    owner: string;
    reputationScore: number;
}

