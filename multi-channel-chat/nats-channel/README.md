# NATS Channel
The NATS channel actor subscribes to a specific topic to listen for incoming chat messages. It then relays
those messages to the API Gateway actor. Additionally, the NATS channel actor can be invoked directly
via the `Outbound` service, which the API Gateway uses to relay messages as they arrive from other channels.

## Compilation
Simply run `wash build` (you'll need `v0.13.0` or later of [wash](https://github.com/wasmcloud/wash) for this) and it will produce the signed wasm file `./build/natschannel_s.wasm`

## Configuration
The NATS channel actor must be linked to a `wasmcloud:messaging` provider. If you use the open source NATS capability provider, then you will need to supply the following configuration values in a link definition:

* `SUBSCRIPTION={topic}` where `{topic}` should be replaced with whatever subject you like. For the Cosmonic walk-through guide, we use the topic `chat.in`. This is the topic on which the channel will accept new incoming messages.

## Usage
The following is a quick overview of how to use this actor.

* To listen for realtime messages coming out of this actor, subscribe to `rtchat.messages`
* Unlike the HTTP actor, you cannot use this channel to request message history (we didn't feel the added complexity illustrated any value for this sample)

## Schema
When you supply new chat messages to this actor, they must conform to the following JSON structure:

```json
{
    "user_name": "some string",
    "body": "also a string"
}
```
