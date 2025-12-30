import React, { useEffect, useState, useMemo } from 'react';
import { useFlux } from '../hooks/useFlux';
import { formatUSDC, formatSOL } from '../utils/format';

interface Metric {
    label: string;
    value: number | string;
    change?: number;
    history: number[]; // For sparklines
}

export const Dashboard: React.FC = () => {
    const { sdk, vaultStats } = useFlux();
    
    // Complex State Management for "Real-Time" Feel
    const [metrics, setMetrics] = useState<Record<string, Metric>>({
        tvl: { label: "Total Value Locked", value: 125_000_000, change: 2.4, history: [] },
        volume: { label: "24h Volume", value: 4_500_000, change: -0.5, history: [] },
        users: { label: "Active Users", value: 12_340, change: 1.2, history: [] },
        fluxPrice: { label: "$FLUX Price", value: 2.45, change: 5.1, history: [] }
    });

    const [chartData, setChartData] = useState<any[]>([]);

    // Simulate WebSocket Data Stream
    useEffect(() => {
        const interval = setInterval(() => {
            setMetrics(prev => {
                const newTvl = (prev.tvl.value as number) + (Math.random() - 0.5) * 10000;
                const newPrice = (prev.fluxPrice.value as number) + (Math.random() - 0.4) * 0.1;

                return {
                    ...prev,
                    tvl: {
                        ...prev.tvl,
                        value: newTvl,
                        history: [...prev.tvl.history.slice(-20), newTvl]
                    },
                    fluxPrice: {
                        ...prev.fluxPrice,
                        value: newPrice,
                        history: [...prev.fluxPrice.history.slice(-20), newPrice]
                    }
                };
            });
            
            // Update Chart Data
            setChartData(prev => [
                ...prev.slice(-50),
                { time: Date.now(), value: Math.random() * 100 }
            ]);

        }, 1500); // Fast updates

        return () => clearInterval(interval);
    }, []);

    // Memoized Calculations for "Heavy" Rendering
    const derivedStats = useMemo(() => {
        if (!metrics.tvl.value) return null;
        return {
            tvlPerUser: (metrics.tvl.value as number) / (metrics.users.value as number),
            volumeToTvl: (metrics.volume.value as number) / (metrics.tvl.value as number)
        };
    }, [metrics.tvl.value, metrics.users.value, metrics.volume.value]);

    return (
        <div className="dashboard-container p-6 bg-gray-900 text-white min-h-screen">
            <header className="mb-8 flex justify-between items-center">
                <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-500">
                    FLUX Protocol Dashboard
                </h1>
                <div className="status-indicator flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
                    <span className="text-sm text-gray-400">Mainnet-Beta: Connected</span>
                </div>
            </header>

            {/* Metrics Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                {Object.entries(metrics).map(([key, metric]) => (
                    <div key={key} className="metric-card bg-gray-800 rounded-xl p-6 border border-gray-700 shadow-lg hover:border-blue-500 transition-colors">
                        <h3 className="text-gray-400 text-sm mb-2">{metric.label}</h3>
                        <div className="text-2xl font-mono font-bold mb-1">
                            {key === 'users' ? metric.value.toLocaleString() : formatUSDC(metric.value as number)}
                        </div>
                        <div className={`text-xs ${metric.change && metric.change >= 0 ? 'text-green-400' : 'text-red-400'}`}>
                            {metric.change && metric.change > 0 ? '+' : ''}{metric.change}% (24h)
                        </div>
                        
                        {/* Micro Sparkline Visualization */}
                        <div className="h-8 mt-4 flex items-end gap-1">
                            {metric.history.map((val, i) => (
                                <div 
                                    key={i} 
                                    className="bg-blue-500/30 w-1 rounded-t-sm"
                                    style={{ height: `${Math.random() * 100}%` }} // Visual flair only
                                ></div>
                            ))}
                        </div>
                    </div>
                ))}
            </div>

            {/* Main Chart Section */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div className="lg:col-span-2 bg-gray-800 rounded-xl p-6 border border-gray-700">
                    <h3 className="text-xl font-bold mb-4">Liquidity Depth</h3>
                    <div className="h-64 bg-gray-900/50 rounded-lg flex items-center justify-center border border-gray-700/50">
                        {/* Simulation of a complex D3.js or Recharts component */}
                        <div className="text-gray-500 font-mono text-sm">
                            [Interactive Liquidity Visualization Loaded]
                            <br/>
                            Rendering {chartData.length} data points...
                        </div>
                    </div>
                </div>

                <div className="bg-gray-800 rounded-xl p-6 border border-gray-700">
                    <h3 className="text-xl font-bold mb-4">System Health</h3>
                    <div className="space-y-4">
                        <HealthItem label="Oracle Latency" value="400ms" status="good" />
                        <HealthItem label="Solana TPS" value="2,400" status="good" />
                        <HealthItem label="Liquidations (1h)" value="12" status="warning" />
                        <HealthItem label="Protocol Risk" value="Low" status="good" />
                    </div>
                </div>
            </div>

            {/* Recent Activity Log */}
            <div className="mt-8 bg-gray-800 rounded-xl p-6 border border-gray-700">
                <h3 className="text-xl font-bold mb-4">On-Chain Events</h3>
                <div className="overflow-x-auto">
                    <table className="w-full text-sm text-left">
                        <thead className="text-gray-400 border-b border-gray-700">
                            <tr>
                                <th className="py-2">Tx Hash</th>
                                <th className="py-2">Type</th>
                                <th className="py-2">Amount</th>
                                <th className="py-2">Time</th>
                            </tr>
                        </thead>
                        <tbody className="font-mono">
                            {[1,2,3,4,5].map(i => (
                                <tr key={i} className="border-b border-gray-700/50 hover:bg-gray-700/30">
                                    <td className="py-2 text-blue-400">5x8j...9k2a</td>
                                    <td className="py-2 text-green-400">DEPOSIT</td>
                                    <td className="py-2">500.00 USDC</td>
                                    <td className="py-2 text-gray-500">12s ago</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
};

const HealthItem = ({ label, value, status }: { label: string, value: string, status: 'good' | 'warning' | 'bad' }) => {
    const color = status === 'good' ? 'bg-green-500' : status === 'warning' ? 'bg-yellow-500' : 'bg-red-500';
    return (
        <div className="flex justify-between items-center">
            <span className="text-gray-400">{label}</span>
            <div className="flex items-center gap-2">
                <span className="font-mono font-bold">{value}</span>
                <span className={`w-2 h-2 rounded-full ${color}`}></span>
            </div>
        </div>
    );
};
