// UpdateVoiceState

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVoiceStateRequest {
    #[prost(fixed64, tag = "1")]
    pub guild_id: u64,
    #[prost(fixed64, tag = "2")]
    pub channel_id: u64,
    #[prost(bool, tag = "3")]
    pub self_mute: bool,
    #[prost(bool, tag = "4")]
    pub self_deaf: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVoiceStateResponse {}
// UpdateStatus

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub shard_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub since: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "3")]
    pub activities: ::std::vec::Vec<update_status_request::ActivityData>,
    #[prost(enumeration = "super::model::presence_data::OnlineStatus", tag = "4")]
    pub status: i32,
    #[prost(bool, tag = "5")]
    pub afk: bool,
}
pub mod update_status_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActivityData {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(
            enumeration = "super::super::model::presence_data::presence_activity_data::ActivityType",
            tag = "2"
        )]
        pub r#type: i32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusResponse {}
// FindUser

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindUserRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindUserResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::std::option::Option<super::model::UserData>,
}
// FindUserMutualGuilds

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserMutualGuildsRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserMutualGuildsResponse {
    #[prost(message, repeated, tag = "1")]
    pub guilds: ::std::vec::Vec<super::model::GuildData>,
}
// FindEmoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub emoji_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindEmojiResponse {
    #[prost(message, optional, tag = "1")]
    pub emoji: ::std::option::Option<super::model::EmojiData>,
}
// GetStats

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsResponse {
    #[prost(uint64, tag = "1")]
    pub guild_count: u64,
    #[prost(uint64, tag = "2")]
    pub user_count: u64,
    #[prost(uint64, tag = "3")]
    pub member_count: u64,
    #[prost(uint64, tag = "4")]
    pub connected_channels: u64,
    #[prost(uint32, tag = "5")]
    pub shard_count: u32,
}
