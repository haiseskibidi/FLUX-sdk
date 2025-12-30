import { PublicKey } from "@solana/web3.js";

export class MockOracle {
    static async setPrice(price: number) {
        console.log(`[MockOracle] Setting price to $${price}`);
        // In a real localnet test, this would write to a local account
    }

    static getProgramId(): PublicKey {
        return new PublicKey("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH"); // Pyth Mock
    }
}

