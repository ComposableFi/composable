import ReactDOM from "react-dom/client";
import { Transfers } from "./Transfers";
import { ExecutorProvider, initialize, Networks } from "../../src";

initialize([{ ...Networks.picasso, wsUrl: "ws://127.0.0.1:9988" }]).then(() => {
  console.log("Substrate React Initialized!");
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ExecutorProvider>
    <Transfers />
  </ExecutorProvider>
);
