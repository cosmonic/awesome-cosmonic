# The Chat Log Actor
This actor expoes a read/write service over the persistent storage of chat messages. It accepts a call
to store a new message, and when it receives a request for the message history it will return those messages in chronological order.

## Compilation
Simply run `wash build` (you'll need `v0.13.0` or later of [wash](https://github.com/wasmcloud/wash) for this) and it will produce the signed wasm file `./build/chatlog_s.wasm`

## Configuration
The chat log actor needs a binding to a `wasmcloud:keyvalue` capability provider. In the Cosmonic walkthrough, we use a Redis capability provider with the following configuration values (note you'll need Redis running locally or in a docker image for this):
* `URL=redis://0.0.0.0:6379`