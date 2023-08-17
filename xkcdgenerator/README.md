# XKCD Generator Application

Click on the button below to launch a bunch of stuff on cosmonic:

[![Deploy on Cosmonic](https://cosmonic.com/badge/deploy.svg)](https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/xkcd-wadm/xkcdgenerator/xkcd-generator-cosmonic.wadm)

The XKCD image generator application is a simple HTTP application that randomly produces random XKCD comics on demand. It illustrates how to build a fully functioning application with a single actor and a few capability providers. You can find the inspiration and architecture diagram for this application in the [Cosmonic documentation](https://cosmonic.com/docs/to-build/image-generator#architecture-and-design).

This example actor makes use of three capabilities:

- `wasmcloud:httpserver` for responding to http requests
- `wasmcloud:httpclient` for fetching http resources from the internet
- `wasmcloud:builtin:numbergen` for generating a random number.

Each time you refresh the page, you should get a random xkcd comic!

## Building this actor

Ensure you have a Rust toolchain and the `wasm32-unknown-unknown` target installed, then simply run `cosmo build` to build and sign this actor. The total size of this WebAssembly module, when compiled with Rust is approximately **219kb**.

## Running this example on Cosmonic

Follow [the guide](https://cosmonic.com/docs/to-build/image-generator?guide=run-it) on Cosmonic's documentation site.

## Watch the build process

Cosmonic developers built this actor [live on stream](https://cosmonic.com/docs/to-build/image-generator?guide=watch-it), including the thought process behind the code here and explanations about the components that went into this application. Check it out!
