#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStreamClientMessage {
    #[prost(oneof = "worker_stream_client_message::Payload", tags = "1, 2, 3, 4")]
    pub payload: ::std::option::Option<worker_stream_client_message::Payload>,
}
pub mod worker_stream_client_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        IdentifyRequest(super::WorkerIdentifyRequest),
        #[prost(message, tag = "2")]
        HeartbeatRequest(super::WorkerHeartbeatRequest),
        #[prost(message, tag = "3")]
        HeartbeatResponse(super::WorkerHeartbeatResponse),
        #[prost(message, tag = "4")]
        DrainRequest(super::WorkerDrainRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStreamServerMessage {
    #[prost(
        oneof = "worker_stream_server_message::Payload",
        tags = "1, 2, 3, 4, 5"
    )]
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
        HeartbeatResponse(super::WorkerHeartbeatResponse),
        #[prost(message, tag = "5")]
        DrainResponse(super::WorkerDrainResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerIdentifyRequest {
    #[prost(string, tag = "1")]
    pub auth_token: std::string::String,
    #[prost(string, tag = "2")]
    pub consumer_group: std::string::String,
    #[prost(string, tag = "3")]
    pub consumer_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerIdentifyResponse {
    #[prost(enumeration = "worker_identify_response::IdentifyStatus", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub consumer_id: std::string::String,
}
pub mod worker_identify_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdentifyStatus {
        Unknown = 0,
        Ok = 1,
        Error = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerHeartbeatRequest {
    #[prost(uint64, tag = "1")]
    pub last_sequence: u64,
    #[prost(string, tag = "2")]
    pub nonce: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerHeartbeatResponse {
    #[prost(string, tag = "1")]
    pub nonce: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerDrainRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerDrainResponse {}
