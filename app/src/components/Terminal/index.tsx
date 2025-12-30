import React from 'react';

export const Terminal: React.FC = () => {
    return (
        <div className="terminal-window">
            <div className="terminal-header">Flux Protocol Command Line Interface v2.1.0</div>
            <div className="terminal-body">
                <span className="prompt">$</span> initializing connection...<br/>
                <span className="success">Connected to Solana Mainnet-Beta</span>
            </div>
        </div>
    );
};

