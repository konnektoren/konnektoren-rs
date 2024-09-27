import { toUserFriendlyAddress } from "https://esm.run/@tonconnect/sdk";
import { TonConnectUI } from "https://esm.run/@tonconnect/ui";

const USE_TEST_NETWORK = true;
const TON_API_URL = "https://testnet.tonapi.io/v2";

let tonConnectUI = null;

export async function initTonWallet(
  manifestUrl,
  onConnectCallback,
  onDisconnectCallback,
) {
  console.log("initTonWallet ", manifestUrl);
  try {
    if (!tonConnectUI) {
      tonConnectUI = new TonConnectUI({
        manifestUrl: manifestUrl,
        buttonRootId: "ton-wallet-button",
      });
    }

    tonConnectUI.onStatusChange(async (wallet) => {
      try {
        if (!wallet) {
          onDisconnectCallback("Not connected");
          return;
        }
        const address = wallet.account.address;
        const balance = await getWalletBalance(address);
        onConnectCallback(address, balance.toString());
      } catch (innerError) {
        console.error("Error in onStatusChange callback:", innerError);
        onConnectCallback("Error", "0");
      }
    });

    // Return the tonConnectUI instance
    return tonConnectUI;
  } catch (outerError) {
    console.error("Error in initTonWallet:", outerError);
    onConnectCallback("Error", "0");
    return null;
  }
}

async function getWalletBalance(address) {
  // Implement this function to fetch the wallet balance
  // You might need to use a TON API or SDK for this
  // For now, we'll return a placeholder value
  return 1000000000; // 1 TON
}

export async function payTonWallet(address, amount) {
  console.log("payTonWallet", address, amount);

  if (!tonConnectUI) {
    throw new Error("TonConnect UI is not initialized");
  }

  try {
    // In TON, 1 TON = 1,000,000,000 nanoTONs
    // The amount should be provided in nanoTONs
    // For example:
    // 1 TON = 1,000,000,000
    // 0.1 TON = 100,000,000
    // 0.01 TON = 10,000,000
    const transaction = {
      validUntil: Math.floor(Date.now() / 1000) + 60 * 20, // Valid for 20 minutes
      network: USE_TEST_NETWORK ? 1 : 0, // 1 for testnet, 0 for mainnet
      from: tonConnectUI.account.address,
      to: address,
      amount: amount.toString(), // Amount in nanoTONs
      // You can add more fields like stateInit, payload, etc. if needed
    };

    const result = await tonConnectUI.sendTransaction(transaction);
    console.log("Transaction sent:", result);
    return result;
  } catch (error) {
    console.error("Error sending transaction:", error);
    throw error;
  }
}
