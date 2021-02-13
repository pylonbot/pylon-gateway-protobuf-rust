// GetGuild

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildResponse {
    #[prost(message, optional, tag = "1")]
    pub guild: ::std::option::Option<super::model::GuildData>,
}
// ListGuildChannels

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildChannelsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildChannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::std::vec::Vec<super::model::ChannelData>,
}
// GetGuildChannel

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildChannelRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildChannelResponse {
    #[prost(message, optional, tag = "1")]
    pub channel: ::std::option::Option<super::model::ChannelData>,
}
// ListGuildMembers

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildMembersRequest {
    #[prost(fixed64, tag = "1")]
    pub after: u64,
    #[prost(uint32, tag = "2")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildMembersResponse {
    #[prost(message, repeated, tag = "1")]
    pub members: ::std::vec::Vec<super::model::MemberData>,
}
// GetGuildMember

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberResponse {
    #[prost(message, optional, tag = "1")]
    pub member: ::std::option::Option<super::model::MemberData>,
}
// FindGuildMember

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindGuildMembersRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub prefix: ::std::option::Option<::std::string::String>,
    #[prost(enumeration = "super::model::presence_data::OnlineStatus", tag = "3")]
    pub status: i32,
    #[prost(uint32, tag = "4")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindGuildMembersResponse {
    #[prost(message, repeated, tag = "1")]
    pub members: ::std::vec::Vec<super::model::MemberData>,
}
// GetGuildMemberPresence

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberPresenceRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberPresenceResponse {
    #[prost(message, optional, tag = "1")]
    pub presence: ::std::option::Option<super::model::PresenceData>,
}
// ListGuildRoles

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildRolesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildRolesResponse {
    #[prost(message, repeated, tag = "1")]
    pub roles: ::std::vec::Vec<super::model::RoleData>,
}
// GetGuildRole

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildRoleRequest {
    #[prost(fixed64, tag = "1")]
    pub role_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildRoleResponse {
    #[prost(message, optional, tag = "1")]
    pub role: ::std::option::Option<super::model::RoleData>,
}
// ListGuildEmojis

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildEmojisRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildEmojisResponse {
    #[prost(message, repeated, tag = "1")]
    pub emojis: ::std::vec::Vec<super::model::EmojiData>,
}
// GetGuildEmoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub emoji_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildEmojiResponse {
    #[prost(message, optional, tag = "1")]
    pub emoji: ::std::option::Option<super::model::EmojiData>,
}
// GetGuildMemberVoiceState

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberVoiceStateRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberVoiceStateResponse {
    #[prost(message, optional, tag = "1")]
    pub voice_state_data: ::std::option::Option<super::model::VoiceStateData>,
}
// ListGuildChannelVoiceStates

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildChannelVoiceStatesRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildChannelVoiceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub voice_states_data: ::std::vec::Vec<super::model::VoiceStateData>,
}
