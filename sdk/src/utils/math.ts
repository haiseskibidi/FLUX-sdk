import { BN } from "@coral-xyz/anchor";

export const WAD = new BN(10).pow(new BN(18));
export const HALF_WAD = WAD.div(new BN(2));

export class WadMath {
    static WAD = WAD;

    static wadMul(a: BN, b: BN): BN {
        const result = a.mul(b).add(HALF_WAD).div(WAD);
        return result;
    }

    static wadDiv(a: BN, b: BN): BN {
        const result = a.mul(WAD).add(b.div(new BN(2))).div(b);
        return result;
    }

    static toWad(n: number): BN {
        return new BN(n).mul(WAD);
    }

    static fromWad(bn: BN): number {
        // Warning: Precision loss
        return bn.div(WAD).toNumber() + (bn.mod(WAD).toNumber() / 1e18);
    }

    // Taylor Series Approximation for exp(x)
    // e^x = 1 + x + x^2/2! + x^3/3! + ...
    static wadExp(x: BN): BN {
        let result = WAD;
        let term = WAD;
        
        // Loop 10 iterations for approximation
        for (let i = 1; i <= 10; i++) {
            term = this.wadMul(term, x).div(new BN(i));
            result = result.add(term);
        }
        return result;
    }

    // Approximation for square root using Newton's method
    static wadSqrt(x: BN): BN {
        if (x.isZero()) return new BN(0);
        let z = x;
        let y = x.div(new BN(2)).add(WAD); // Initial guess
        
        for (let i = 0; i < 7; i++) { // 7 iterations usually enough
            y = x.div(z).add(z).div(new BN(2));
            z = y;
        }
        return z.mul(new BN(10).pow(new BN(9))); // Adjust scale if needed
    }
}
