export * from './core/connection';
export * from './core/transaction-builder';
export * from './constants/addresses';
export * from './types';
export * from './utils/math';

import { Connection, PublicKey } from '@solana/web3.js';
import { FluxConnection } from './core/connection';

/**
 * Initialize the FLUX Protocol SDK (v2.1.0-beta)
 * @param endpoint RPC Endpoint
 * @returns FluxConnection instance
 */
export function initFlux(endpoint: string): FluxConnection {
    return new FluxConnection(new Connection(endpoint));
}
