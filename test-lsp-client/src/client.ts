import { log } from "console";
import { open, openSync, readFileSync } from "fs";
import { exit } from "process";
import { createServerSocketTransport } from "vscode-jsonrpc/node";
import {
  createProtocolConnection,
  DefinitionRequest,
  DidOpenTextDocumentNotification,
  ExitNotification,
  HoverParams,
  HoverRequest,
  InitializedNotification,
  InitializeParams,
  InitializeRequest,
  ShutdownRequest,
  TextDocumentItem,
} from "vscode-languageserver-protocol";

const folder_path =
  "/Users/julind/Projects/firestore-rules-lsp/tree-sitter-firestore_rules/examples";

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
    rootUri: "file://" + folder_path,
    processId: process.pid,
    capabilities: {},
    workspaceFolders: null,
  };

  connection.listen();
  log("Listening");

  await connection.sendRequest(InitializeRequest.type, init);
  log("Initialized server");

  await connection.sendNotification(InitializedNotification.type, {});
  log("Initialized notif sent");

  const file_path = folder_path + "/example.rules";
  const file_contents = readFileSync(file_path);
  const uri = "file://" + file_path;

  await connection.sendNotification(DidOpenTextDocumentNotification.type, {
    textDocument: {
      uri: uri,
      languageId: "firestore_rules",
      text: file_contents.toString(),
      version: 0,
    },
  });
  /* 
    //----
  
    let hover1 = await connection.sendRequest(HoverRequest.type, {
      textDocument: {
        uri: uri,
      },
      position: {
        line: 32,
        character: 44,
      },
    } as HoverParams);
  
    log(hover1);
  
    let gdef1 = await connection.sendRequest(DefinitionRequest.type, {
      textDocument: {
        uri: uri,
      },
      position: {
        line: 32,
        character: 44,
      },
    });
  
    log(gdef1);
  
    //----
  
    let hover2 = await connection.sendRequest(HoverRequest.type, {
      textDocument: {
        uri: uri,
      },
      position: {
        line: 24,
        character: 22,
      },
    } as HoverParams);
  
    log(hover2); */

  let gdef2 = await connection.sendRequest(DefinitionRequest.type, {
    textDocument: {
      uri: uri,
    },
    position: {
      line: 24,
      character: 22,
    },
  });

  log(gdef2);


  await connection.sendRequest(ShutdownRequest.type);
  log("Shutdown request sent");

  await connection.sendNotification(ExitNotification.type);
  log("Exit notification sent");

  exit(0);
}

async function sleep(time: number) {
  await new Promise((resolve) => setTimeout(resolve, time));
}
