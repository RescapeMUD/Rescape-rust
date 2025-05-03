/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_HELIUS_DEVNET_RPC: string;
    readonly VITE_HELIUS_DEVNET_WS: string;
    readonly VITE_HELIUS_MAINNET_RPC: string;
    readonly VITE_HELIUS_MAINNET_WS: string;
  }
  
  interface ImportMeta {
    readonly env: ImportMetaEnv;
  }