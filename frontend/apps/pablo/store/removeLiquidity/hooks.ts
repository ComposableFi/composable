import useStore from "@/store/useStore";

export const useRemoveLiquidityState = () => {
  const { removeLiquidity } = useStore();

  return removeLiquidity;
};
