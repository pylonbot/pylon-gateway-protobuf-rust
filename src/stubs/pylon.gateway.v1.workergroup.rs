/// Client -> Server messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStreamClientMessage {
    #[prost(oneof = "worker_stream_client_message::Payload", tags = "1, 2, 3")]
    pub payload: ::std::option::Option<worker_stream_client_message::Payload>,
}
pub mod worker_stream_client_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        IdentifyRequest(super::WorkerIdentifyRequest),
        #[prost(message, tag = "2")]
        HeartbeatAck(super::WorkerHeartbeatAck),
        #[prost(message, tag = "3")]
        DrainRequest(super::WorkerDrainRequest),
    }
}
/// Server -> Client messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStreamServerMessage {
    #[prost(oneof = "worker_stream_server_message::Payload", tags = "1, 2, 3, 4")]
    pub payload: ::std::option::Option<worker_stream_server_message::Payload>,
}
pub mod worker_stream_server_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        IdentifyResponse(super::WorkerIdentifyResponse),
        #[prost(message, tag = "2")]
        EventEnvelope(super::super::super::super::discord::v1::event::EventEnvelope),
        #[prost(message, tag = "3")]
        HeartbeatRequest(super::WorkerHeartbeatRequest),
        #[prost(message, tag = "4")]
        StreamClosed(super::WorkerStreamClosed),
    }
}
/// Identification is the first message sent
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerIdentifyRequest {
    #[prost(string, tag = "1")]
    pub auth_token: std::string::String,
    #[prost(string, tag = "2")]
    pub consumer_group: std::string::String,
    #[prost(string, tag = "3")]
    pub consumer_id: std::string::String,
    #[prost(string, tag = "4")]
    pub router_ticket: std::string::String,
}
/// Router tickets are used for robust reconnections
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerIdentifyResponse {
    #[prost(string, tag = "1")]
    pub router_ticket: std::string::String,
}
/// Heartbeats are used to keep check on clients and acknowledge received events
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerHeartbeatRequest {
    #[prost(string, tag = "1")]
    pub nonce: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerHeartbeatAck {
    #[prost(string, tag = "1")]
    pub nonce: std::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
}
/// Clients can request to drain their connections
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerDrainRequest {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
/// The server may close the connection with a reason
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStreamClosed {
    #[prost(enumeration = "worker_stream_closed::CloseReason", tag = "1")]
    pub reason: i32,
}
pub mod worker_stream_closed {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CloseReason {
        Unknown = 0,
        HeartbeatTimeout = 1,
        InvalidIdentity = 2,
        DrainComplete = 3,
        RequestedReconnect = 4,
    }
}
