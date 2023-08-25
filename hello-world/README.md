# Hello world

Click on the button below to launch Hello World on Cosmonic Now:

[<img src="https://cosmonic.com/badge/deploy.svg" alt="Deploy on Cosmonic" width="400">](https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/main/hello-world/hello-cosmonic.wadm)

This actor is the "Hello, World" of Cosmonic applications. It uses a single capability, HTTP server, and returns a string as a response. This project is actually a template, and if you'd like to try it for yourself, install the [cosmo cli](https://cosmonic.com/docs/getting-started/quickstart) and run:

```
cosmo new actor --git https://github.com/cosmonic/things-to-build --path hello-world/rust hello
cosmo launch -p hello
```

