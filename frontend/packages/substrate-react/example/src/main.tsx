import ReactDOM from 'react-dom/client'
import { Transfers } from './Transfers';
import { ExecutorProvider } from '../../src';
import { DotSamaContextProvider } from '../../src/dotsama/DotSamaContext';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <DotSamaContextProvider appName='DEMO_APP' supportedParachains={[
    { chainId: "picasso", rpcUrl: "ws://127.0.0.1:9988", rpc: undefined, types: undefined }
  ]}>
    <ExecutorProvider>
      <Transfers />
    </ExecutorProvider>
  </DotSamaContextProvider>
)
