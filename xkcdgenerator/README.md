# XKCD Generator Application

The XKCD image generator application is a simple HTTP application that randomly produces random XKCD comics on demand. It illustrates how to build a fully functioning application with a single actor and a few capability providers. You can find the inspiration and architecture diagram for this application in the [Cosmonic documentation](https://cosmonic.com/docs/to-build/image-generator).

This example actor makes use of three capabilities:

- `wasmcloud:httpserver` for responding to http requests
- `wasmcloud:httpclient` for fetching http resources from the internet
- `wasmcloud:builtin:numbergen` for generating a random number.

Each time you refresh the page, you should get a random xkcd comic!

## Building this actor

Ensure you have a Rust toolchain and the `wasm32-unknown-unknown` target installed. If you have [wash](https://wasmcloud.dev/overview/installation/) version `0.13.0` or greater, you can run `wash build`. Otherwise, simply run `make` to build and sign this actor. The total size of this WebAssembly module, when compiled with Rust is approximately **219kb**.

## Running this example on Cosmonic

Navigate to the [Logic View](https://cosmonic.com/docs/category/logic-view) in your Constellation and click Launch Actor in the left-hand toolbar. Select the sample XKCD actor from the dropdown and launch a few instances on a host.
![Launch XKCD Actor](./launch.png)

Then, using the Launch Provider modal, launch both the **HTTP Server (Wormhole)** and then the **HTTP Client** capability providers to satisfy the required capabilities. After everything is launched, drag from the `wasmcloud:httpserver` bubble on the XKCD actor to the HTTP Server provider and from the `wasmcloud:httpclient` bubble to the HTTP Client provider. Once those are linked, you can create a [wormhole](https://cosmonic.com/docs/faq/glossary#wormhole) and access your XKCD comic generator publicly.

## Watch the build process

Cosmonic developers built this actor live on stream, including the thought process behind the code here and explanations about the components that went into this application.
//insert
