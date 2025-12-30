import { useEffect, useState, useCallback, useRef } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { initFlux, FluxConnection } from '@flux-protocol/sdk';
import { PublicKey } from '@solana/web3.js';

interface FluxState {
    sdk: FluxConnection | null;
    isInitializing: boolean;
    error: string | null;
    vaultStats: any | null;
    userPosition: any | null;
}

export const useFlux = () => {
    const { connection } = useConnection();
    const { publicKey } = useWallet();
    const [state, setState] = useState<FluxState>({
        sdk: null,
        isInitializing: true,
        error: null,
        vaultStats: null,
        userPosition: null
    });

    // Ref to prevent double-initialization in strict mode
    const initializedRef = useRef(false);

    useEffect(() => {
        if (!connection || initializedRef.current) return;
        
        const initialize = async () => {
            try {
                console.log("[useFlux] Initializing SDK...");
                const sdk = initFlux(connection.rpcEndpoint);
                
                setState(prev => ({ ...prev, sdk, isInitializing: false }));
                initializedRef.current = true;
                
                // Start background polling
                startDataPolling(sdk);
                
            } catch (err: any) {
                console.error("SDK Init Failed:", err);
                setState(prev => ({ 
                    ...prev, 
                    isInitializing: false, 
                    error: err.message 
                }));
            }
        };

        initialize();
    }, [connection]);

    // Data Polling Logic
    const startDataPolling = useCallback((sdk: FluxConnection) => {
        // Poll Global Stats
        const pollStats = async () => {
            try {
                // Mock call
                const vaultAddr = new PublicKey("FluxCore1111111111111111111111111111111111111"); 
                const stats = await sdk.getVaultState(vaultAddr, { useCache: true });
                setState(prev => ({ ...prev, vaultStats: stats }));
            } catch (e) {
                console.warn("Polling error:", e);
            }
        };

        const intervalId = setInterval(pollStats, 5000); // Every 5s
        pollStats(); // Initial call

        return () => clearInterval(intervalId);
    }, []);

    // User Position Fetching
    useEffect(() => {
        if (!state.sdk || !publicKey) return;

        const fetchUserPosition = async () => {
            // ... complex logic to fetch user PDAs ...
            console.log("Fetching user position for", publicKey.toBase58());
            
            setState(prev => ({
                ...prev,
                userPosition: {
                    deposited: 1000,
                    borrowed: 500,
                    healthFactor: 1.5
                }
            }));
        };

        fetchUserPosition();
    }, [state.sdk, publicKey]);

    return state;
};
