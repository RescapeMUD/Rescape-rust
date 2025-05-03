export function getHeliusEndpoints(cluster: 'devnet' | 'mainnet') {
    return {
        rpc:
            cluster === 'devnet'
                ? import.meta.env.VITE_HELIUS_DEVNET_RPC
                : import.meta.env.VITE_HELIUS_MAINNET_RPC,
        ws:
            cluster === 'devnet'
                ? import.meta.env.VITE_HELIUS_DEVNET_WS
                : import.meta.env.VITE_HELIUS_MAINNET_WS
    };
}