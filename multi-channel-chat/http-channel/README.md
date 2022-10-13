# HTTP Channel Actor
The HTTP channel actor exposes a restful interface that allows for the querying of the message log and the submission of new messages. The following resources/URLs comprise this API:

* `GET /messages` - Returns a list of all messages in the application in chronological order
* `POST /messages` - Adds a new message to the chat log and relays it to all other active channels. The new message must be in the body of the POST.

When posting a new message, it must have the following JSON structure:

```json
{
    "user_name": "some string",
    "body": "some other string"
}
```

## Compilation
Simply run `wash build` (you'll need `v0.13.0` or later of [wash](https://github.com/wasmcloud/wash) for this) and it will produce the signed wasm file `./build/chatlog_s.wasm`

## Configuration
The HTTP channel actor must be linked to a `wasmcloud:httpserver` capability provider. For the Cosmonic walkthrough guide, we used a Cosmonic wormhole-aware capability provider. If you're running this on your own infrastructure, you can provide the following value to start the web server on the port of your choice:
* `address=0.0.0.0:{port}` where `{port}` is where you want to run the server, e.g. `address=0.0.0.0:8080`.