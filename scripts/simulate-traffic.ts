import { Connection, Keypair } from "@solana/web3.js";
import { initFlux } from "../sdk/src";

const SIMULATION_DURATION = 60 * 1000; // 1 minute
const REQUESTS_PER_SECOND = 10;

async function simulateTraffic() {
    console.log(`Starting traffic simulation: ${REQUESTS_PER_SECOND} rps for ${SIMULATION_DURATION}ms`);
    
    const flux = initFlux("http://localhost:8899");
    const startTime = Date.now();
    let requests = 0;

    const interval = setInterval(async () => {
        if (Date.now() - startTime > SIMULATION_DURATION) {
            clearInterval(interval);
            console.log(`Simulation complete. Total requests: ${requests}`);
            process.exit(0);
        }

        // Simulate a batch of fetch requests
        const vaultId = Keypair.generate().publicKey; // Random vault for simulation
        try {
            // In a real load test, we wouldn't await every single one sequentially in the interval
            flux.getVaultState(vaultId).catch(() => {}); // Fire and forget
            requests++;
            process.stdout.write(`\rRequests sent: ${requests}`);
        } catch (e) {
            // Ignore errors for load testing
        }

    }, 1000 / REQUESTS_PER_SECOND);
}

simulateTraffic();

