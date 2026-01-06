# Firestore Rules Language Server Provider

#### A LSP implementation for the [Firestore Security Rules Language](https://firebase.google.com/docs/rules/rules-language#firestore)

This LSP implementation is standalone and does not require the Firebase Admin SDK toolchain to parse a firestore rules document. It uses its own tree sitter based grammar currently maintaned in the [tree](https://github.com/JulindM/firestore-rules-lsp/tree/main/tree-sitter-firestore_rules).

---

### Usage

The server supports communicating over STDIO or TCP. Starting the server without any parameters will start it over STDIO.

#### Over TCP:

To start the server over TCP, you need to have a client started and already bound to `<PORT_NUMBER>` awaiting the server capabilities to be sent. If this is the case then run

```sh
$ firestore-rules-lsp --socket=<PORT_NUMBER>
```

for the lsp to initialize the connection

### Current capabilites

- Dynamic Type inference
- Text Document Synchronization
- Semantic Tokens Provider
- Definition Provider
- Publish Diagnostics Notifications
- Hover
- Autocomplete
- Linting
  - Diagnostics of variable usages that no definiton was found for them
  - Diagnostics of function calls that no definition was found for them
  - Diagnostics on rules that do not resolve to a boolean value

### Also check out

Currently there is a VSCode extension in the works that simply wraps around this LSP to enable it in VSCode.

_Check it out [here](https://github.com/JulindM/firestore-rules-lsp-vscode)._
