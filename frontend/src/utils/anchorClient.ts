import { AnchorProvider, Program, Idl, setProvider } from "@coral-xyz/anchor";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { useMemo } from "react";
import { PublicKey } from "@solana/web3.js";

import idl from "../idl/rescape_protocol.json";
import { RescapeProtocol } from "../types/rescape_protocol";

// 👇 Replace with your actual deployed program ID
const PROGRAM_ID = new PublicKey("ExXC4aBe6MtJ9LdMP8wB9536KPkGCd2bdp1og4BzTYWz");

export const useAnchorProgram = () => {
  const { connection } = useConnection();
  const wallet = useWallet();

  const provider = useMemo(() => {
    if (!wallet || !wallet.publicKey || !wallet.signTransaction) return null;

    return new AnchorProvider(connection, wallet as any, {
      commitment: "confirmed",
    });
  }, [wallet, connection]);

  const program = useMemo(() => {
    if (!provider) return null;

    setProvider(provider);

    return new Program<RescapeProtocol>(idl as Idl, provider);
  }, [provider]);

  return program;
};
