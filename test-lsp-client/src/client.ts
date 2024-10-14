import { log } from "console";
import { exit } from "process";
import { createServerSocketTransport } from "vscode-jsonrpc/node";
import {
  createProtocolConnection,
  ExitNotification,
  InitializedNotification,
  InitializeParams,
  InitializeRequest,
  ShutdownRequest,
} from "vscode-languageserver-protocol";

run().then(
  () => exit(0),
  (err) => {
    log(err);
    return exit(1);
  }
);

async function run(): Promise<void> {
  debugger;
  const messages = createServerSocketTransport(1234);

  const connection = createProtocolConnection(messages[0], messages[1]);

  const init: InitializeParams = {
    rootUri: "file:///",
    processId: process.pid,
    capabilities: {},
    workspaceFolders: null,
  };

  connection.listen();
  log("Listening");

  await sleep(1000);

  await connection.sendRequest(InitializeRequest.type, init);
  log("Initialized server");

  await sleep(1000);

  await connection.sendNotification(InitializedNotification.type, {});
  log("Initialized notif sent");

  await sleep(3000);

  await connection.sendRequest(ShutdownRequest.type);
  log("Shutdown request sent");

  await connection.sendNotification(ExitNotification.type);
  log("Exit notification sent");

  exit(0);
}

async function sleep(time: number) {
  await new Promise((resolve) => setTimeout(resolve, time));
}
