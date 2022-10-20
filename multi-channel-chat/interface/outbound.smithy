metadata package = [ 
  { 
    namespace: "com.cosmonic.samples.mcchat.outbound", 
    crate: "outbound_crate" 
  }
]

namespace com.cosmonic.samples.mcchat.outbound

use org.wasmcloud.model#wasmbus

@wasmbus( actorReceive: true )
service Outbound {
  version: "0.1",
  operations: [PublishMessage]
}

operation PublishMessage {
    input: OutboundMessage
    output: Boolean
}

structure OutboundMessage {
     @required
    sourceUser: String

    @required
    sourceChannel: String

    @required
    body: String
}
