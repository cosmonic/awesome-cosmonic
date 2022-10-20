metadata package = [ 
  { 
    namespace: "com.cosmonic.samples.mcchat.chatlog", 
    crate: "chatlog_crate" 
  }
]

namespace com.cosmonic.samples.mcchat.chatlog

use org.wasmcloud.model#wasmbus

@wasmbus( actorReceive: true )
service Chatlog {
  version: "0.1",
  operations: [WriteMessage, GetMessages]
}

operation WriteMessage {
    input: CanonicalChatMessage
    output: WriteMessageResponse
}

operation GetMessages {
    output: MessagesList
}

structure WriteMessageResponse {
    @required
    accepted: Boolean,

    reason: String
}

structure CanonicalChatMessage {
    @required
    id: String

    @required
    sourceUser: String

    @required
    channelName: String

    @required
    body: String    
}

list MessagesList {
    member: CanonicalChatMessage
}
