export const detectInstalledWallet = () => {
  return !!window?.web3?.currentProvider;
};
