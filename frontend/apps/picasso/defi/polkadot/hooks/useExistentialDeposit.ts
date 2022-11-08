import { useStore } from "@/stores/root";
import { useTransfer } from "@/defi/polkadot/hooks/useTransfer";

export const useExistentialDeposit = () => {
  const { from, balance, to } = useTransfer();
  const tokenId = useStore((state) => state.transfers.selectedToken);
  const getFeeToken = useStore((state) => state.transfers.getFeeToken);
  const { existentialDeposit } = useStore((state) => state.transfers);

  return {
    balance,
    tokenId,
    from,
    to,
    existentialDeposit,
    feeToken: getFeeToken(from),
  };
};
