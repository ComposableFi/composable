export const getSigner = async (
  applicationName: string,
  address: string
): Promise<any> => {
  const extensionPackage = await import('@polkadot/extension-dapp');
  const { web3FromAddress, web3Enable } = extensionPackage;
  await web3Enable(applicationName);
  const injector = await web3FromAddress(address);
  return injector.signer;
};
