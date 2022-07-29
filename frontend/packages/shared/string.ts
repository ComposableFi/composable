export const getShortAddress = (address: string) => {
  const length = address.length;
  return address.substring(0, 3) + ".." + address.substring(length-5)
};