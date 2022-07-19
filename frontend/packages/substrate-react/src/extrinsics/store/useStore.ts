import create from 'zustand';
import extrinsicsSlice from './extrinsics/extrinsics.slice';

const useStore = create(set => ({
  ...extrinsicsSlice(set),
}));

export default useStore;
