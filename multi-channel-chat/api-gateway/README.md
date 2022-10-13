# API Gateway Actor
The API gateway actor is a pure actor-to-actor component. It accepts calls from any channel using the [Chatlog](../interface/chatlog.smithy) interface. It then relays those same messages to the real chat log actor and invokes a function from the [Outbound](../interface/outbound.smithy)
interface on all other channels to inform them of the new message.

## Compilation
Simply run `wash build` (you'll need `v0.13.0` or later of [wash](https://github.com/wasmcloud/wash) for this) and it will produce the signed wasm file `./build/apigateway_s.wasm`

## Configuration
This actor requires no link definitions or configuration, simply start this actor in any host in a lattice