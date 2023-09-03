import { useInkathon } from "@scio-labs/use-inkathon";
import { detectInstalledWallet } from "../utils/generalUtils.js";
import { deployContract } from "../utils/contract.js";
import { useEffect } from "react";

const WalletConnectButton = () => {
  const {
    connect,
    disconnect,
    isConnected,
    activeAccount,
    activeSigner,
    provider,
    isInitializing,
  } = useInkathon();

  const handleWalletState = () => {
    if (isConnected) {
      disconnect();
      console.log("Wallet disconnected");
    } else {
      connect().then(() => console.log("Connected ᕦ( ͡° ͜ʖ ͡°)ᕤ"));
    }
  };

  useEffect(() => {
    console.warn(import.meta.env.VITE_ENABLE_TEST_DEPLOY)
  }, [provider]);

  const handleTestDeploy = () => {
    deployContract(provider, activeAccount.address, activeSigner, null).then(() =>
      console.log("Done"),
    );
  };

  const connectionStatusText = isConnected ? "Disconnect" : "Connect wallet";

  const walletExtensionExists = detectInstalledWallet();

  return (
    <>
      {import.meta.env.VITE_ENABLE_TEST_DEPLOY && (
        <button onClick={handleTestDeploy}>Test deploy</button>
      )}
      {walletExtensionExists ? (
        <button
          onClick={handleWalletState}
          className="bg-[#0D4948] hover:bg-[#238880] text-white font-bold py-2 px-4 rounded-3xl"
          disabled={isInitializing}
        >
          {connectionStatusText}
        </button>
      ) : (
        <a href={"https://www.talisman.xyz/"} target="_blank" rel="noreferrer">
          <button className="bg-[#0D4948] hover:bg-[#238880] text-white font-bold py-2 px-4 rounded-3xl">
            Install Wallet
          </button>
        </a>
      )}
    </>
  );
};

export default WalletConnectButton;
