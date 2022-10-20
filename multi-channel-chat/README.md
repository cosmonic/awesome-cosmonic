# Multi-Channel Chat Application
This application is designed to be used as an illustration of some of the incredibly powerful things you can do with wasmCloud when hosted on Cosmonic. 

For the full guide and walkthrough for this application, check out the relevant [Cosmonic docs](https://cosmonic.com/docs/to-build/multi-channel-chat).

## Components
The chat application is made up of the following components (_all of which are actors_):

* [API Gateway](./api-gateway/README.md) - The API gateway used by all channels
* [Chatlog](./chatlog/README.md) - The chat log actor, performs read/write operations on message storage
* [HTTP Channel](./http-channel/README.md) - The HTTP channel actor. Accepts HTTP requests and relays them to the API gateway
* [NATS Channel](./nats-channel/README.md) - The NATS channel actor. Accepts new message requests and publishes messages as they arrive in real-time

The shared schema definition for the `Outbound` service definition and the `Chatlog` service definition can be found in the [interface](./interface/README.md) directory.