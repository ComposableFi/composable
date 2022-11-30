{ pkgs }:
{
  mkCollator = { command, name ? "alice", rpc_port ? 32200,  ws_port ? 9988} : {
          command = command;
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "alice";
          rpc_port = 32200;
          validator = true;
          ws_port = 9988;
  };
}