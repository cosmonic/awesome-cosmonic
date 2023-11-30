# HTTP File Server

This actor (we like to call it "Little Blobby Tables") is a simple file server showing the basic
CRUD operations of the `wasmcloud:blobstore` contract.

Not only is this actor an example, it is also a fully-functional, HTTP-based fileserver that can be
fronted with any HTTP server implementation and any blobstore implementation (i.e. you could store
the uploaded files on a filesystem or in an s3 compatible store). It also has a full example of
integration testing for the actor

## Required Capability Claims

1. `wasmcloud:httpserver` to receive http requests
2. `wasmcloud:blobstore` to save the image to a blob
3. `wasmcloud:builtin:logging` so the actor can log

## Running this example

Click on the button below to launch the XKCD Generator on Cosmonic Now:

[![Deploy on Cosmonic](https://cosmonic.com/badge/deploy.svg)](https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/http-file-server/http-file-server.wadm.yaml)

Once everything is up and running, you can run through all of the operations by following the
annotated commands below:

```bash
export MY_WORMHOLE="your-wormhole-goes-here.cosmonic.app"

# to upload a file
export MESSAGE="Hello there\!"
curl -w "%{http_code}" -H 'Content-Type: text/plain' "https://$MY_WORMHOLE.cosmonic.app/myfile.txt" -d "$MESSAGE"

# to download a file
curl "https://$MY_WORMHOLE.cosmonic.app/myfile.txt"

# to edit a file, post again
export MESSAGE="$MESSAGE General Kenobi\!"
curl -w "%{http_code}" -H 'Content-Type: text/plain' "https://$MY_WORMHOLE.cosmonic.app/myfile.txt" -d "$MESSAGE"

# download the file again
curl "https://$MY_WORMHOLE.cosmonic.app/myfile.txt"
```
