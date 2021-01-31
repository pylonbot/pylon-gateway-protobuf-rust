#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestError {
    #[prost(oneof = "rest_error::ErrorType", tags = "1, 2, 3, 4, 5")]
    pub error_type: ::std::option::Option<rest_error::ErrorType>,
}
pub mod rest_error {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnknownError {
        #[prost(uint32, tag = "1")]
        pub http_status: u32,
        #[prost(uint32, tag = "2")]
        pub code: u32,
        #[prost(string, tag = "3")]
        pub message: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationError {
        #[prost(string, tag = "1")]
        pub message: std::string::String,
        #[prost(message, repeated, tag = "2")]
        pub fields: ::std::vec::Vec<validation_error::Field>,
    }
    pub mod validation_error {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Field {
            #[prost(string, tag = "1")]
            pub path: std::string::String,
            #[prost(string, tag = "2")]
            pub code: std::string::String,
            #[prost(string, tag = "3")]
            pub message: std::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceNotFound {
        #[prost(uint32, tag = "1")]
        pub code: u32,
        #[prost(string, tag = "2")]
        pub message: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessDenied {
        #[prost(uint32, tag = "1")]
        pub code: u32,
        #[prost(string, tag = "2")]
        pub message: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateLimited {
        #[prost(bool, tag = "1")]
        pub global: bool,
        #[prost(uint32, tag = "2")]
        pub retry_at: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ErrorType {
        #[prost(message, tag = "1")]
        UnknownError(UnknownError),
        #[prost(message, tag = "2")]
        ValidationError(ValidationError),
        #[prost(message, tag = "3")]
        ResourceNotFound(ResourceNotFound),
        #[prost(message, tag = "4")]
        AccessDenied(AccessDenied),
        #[prost(message, tag = "5")]
        RateLimited(RateLimited),
    }
}
// Modify Guild

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub region: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub verification_level: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub default_message_notifications: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub explicit_content_filter: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub afk_channel_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "7")]
    pub afk_timeout: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub icon: ::std::option::Option<::std::vec::Vec<u8>>,
    #[prost(message, optional, tag = "9")]
    pub owner_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "10")]
    pub splash: ::std::option::Option<::std::vec::Vec<u8>>,
    #[prost(message, optional, tag = "11")]
    pub banner: ::std::option::Option<::std::vec::Vec<u8>>,
    #[prost(message, optional, tag = "12")]
    pub system_channel_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "13")]
    pub rules_channel_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "14")]
    pub public_updates_channel_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "15")]
    pub preferred_locale: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildResponse {
    #[prost(oneof = "modify_guild_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_guild_response::Response>,
}
pub mod modify_guild_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub guild: ::std::option::Option<super::super::model::GuildData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Create Guild Channel

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildChannelRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(enumeration = "super::model::channel_data::ChannelType", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub topic: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub bitrate: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub user_limit: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub rate_limit_per_user: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub position: ::std::option::Option<u32>,
    #[prost(message, repeated, tag = "8")]
    pub permission_overwrites:
        ::std::vec::Vec<super::model::channel_data::ChannelPermissionOverwriteData>,
    #[prost(message, optional, tag = "9")]
    pub parent_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "10")]
    pub nsfw: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildChannelResponse {
    #[prost(oneof = "create_guild_channel_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_guild_channel_response::Response>,
}
pub mod create_guild_channel_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub channel: ::std::option::Option<super::super::model::ChannelData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Guild Channel Positions

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildChannelPositionsRequest {
    #[prost(message, repeated, tag = "1")]
    pub channel_positions: ::std::vec::Vec<modify_guild_channel_positions_request::ChannelPosition>,
}
pub mod modify_guild_channel_positions_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelPosition {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(uint32, tag = "2")]
        pub position: u32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildChannelPositionsResponse {
    #[prost(
        oneof = "modify_guild_channel_positions_response::Response",
        tags = "1, 2"
    )]
    pub response: ::std::option::Option<modify_guild_channel_positions_response::Response>,
}
pub mod modify_guild_channel_positions_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Add Guild Member

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGuildMemberRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(string, tag = "2")]
    pub access_token: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub nick: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub roles: ::std::option::Option<super::model::SnowflakeListValue>,
    #[prost(message, optional, tag = "5")]
    pub mute: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub deaf: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGuildMemberResponse {
    #[prost(oneof = "add_guild_member_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<add_guild_member_response::Response>,
}
pub mod add_guild_member_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(bool, tag = "1")]
        pub added: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Guild Member

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildMemberRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(message, optional, tag = "2")]
    pub nick: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub roles: ::std::option::Option<super::model::SnowflakeListValue>,
    #[prost(message, optional, tag = "4")]
    pub mute: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub deaf: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub channel_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildMemberResponse {
    #[prost(oneof = "modify_guild_member_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_guild_member_response::Response>,
}
pub mod modify_guild_member_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Modify Current User Nick

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyCurrentUserNickRequest {
    #[prost(message, optional, tag = "1")]
    pub nick: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyCurrentUserNickResponse {
    #[prost(oneof = "modify_current_user_nick_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_current_user_nick_response::Response>,
}
pub mod modify_current_user_nick_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Add Guild Member Role

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGuildMemberRoleRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(fixed64, tag = "2")]
    pub role_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGuildMemberRoleResponse {
    #[prost(oneof = "add_guild_member_role_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<add_guild_member_role_response::Response>,
}
pub mod add_guild_member_role_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Remove Guild Member Role

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildMemberRoleRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(fixed64, tag = "2")]
    pub role_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildMemberRoleResponse {
    #[prost(oneof = "remove_guild_member_role_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<remove_guild_member_role_response::Response>,
}
pub mod remove_guild_member_role_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Remove Guild Member

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildMemberRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildMemberResponse {
    #[prost(oneof = "remove_guild_member_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<remove_guild_member_response::Response>,
}
pub mod remove_guild_member_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Guild Bans

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildBansRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildBansResponse {
    #[prost(oneof = "get_guild_bans_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_bans_response::Response>,
}
pub mod get_guild_bans_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub bans: ::std::vec::Vec<super::super::model::GuildBanData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Get Guild Ban

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildBanRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildBanResponse {
    #[prost(oneof = "get_guild_ban_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_ban_response::Response>,
}
pub mod get_guild_ban_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub ban: ::std::option::Option<super::super::model::GuildBanData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Create Guild Ban

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildBanRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(message, optional, tag = "2")]
    pub delete_message_days: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildBanResponse {
    #[prost(oneof = "create_guild_ban_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_guild_ban_response::Response>,
}
pub mod create_guild_ban_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Remove Guild Ban

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildBanRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGuildBanResponse {
    #[prost(oneof = "remove_guild_ban_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<remove_guild_ban_response::Response>,
}
pub mod remove_guild_ban_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Create Guild Role

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildRoleRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub permissions: ::std::option::Option<super::model::PermissionsValue>,
    #[prost(message, optional, tag = "3")]
    pub color: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub hoist: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub mentionable: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildRoleResponse {
    #[prost(oneof = "create_guild_role_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_guild_role_response::Response>,
}
pub mod create_guild_role_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub role: ::std::option::Option<super::super::model::RoleData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Guild Role Positions

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildRolePositionsRequest {
    #[prost(message, repeated, tag = "1")]
    pub role_positions: ::std::vec::Vec<modify_guild_role_positions_request::RolePosition>,
}
pub mod modify_guild_role_positions_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RolePosition {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(uint32, tag = "2")]
        pub position: u32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildRolePositionsResponse {
    #[prost(
        oneof = "modify_guild_role_positions_response::Response",
        tags = "1, 2"
    )]
    pub response: ::std::option::Option<modify_guild_role_positions_response::Response>,
}
pub mod modify_guild_role_positions_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub roles: ::std::vec::Vec<super::super::model::RoleData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Guild Role

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildRoleRequest {
    #[prost(fixed64, tag = "1")]
    pub role_id: u64,
    #[prost(message, optional, tag = "2")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub permissions: ::std::option::Option<super::model::PermissionsValue>,
    #[prost(message, optional, tag = "4")]
    pub color: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub hoist: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub mentionable: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildRoleResponse {
    #[prost(oneof = "modify_guild_role_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_guild_role_response::Response>,
}
pub mod modify_guild_role_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub role: ::std::option::Option<super::super::model::RoleData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Delete Guild Role

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuildRoleRequest {
    #[prost(fixed64, tag = "1")]
    pub role_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuildRoleResponse {
    #[prost(oneof = "delete_guild_role_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_guild_role_response::Response>,
}
pub mod delete_guild_role_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Guild Prune Count

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildPruneCountRequest {
    #[prost(message, optional, tag = "1")]
    pub days: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub include_roles: ::std::option::Option<super::model::SnowflakeListValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildPruneCountResponse {
    #[prost(oneof = "get_guild_prune_count_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_prune_count_response::Response>,
}
pub mod get_guild_prune_count_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Begin Guild Prune

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginGuildPruneRequest {
    #[prost(message, optional, tag = "1")]
    pub days: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub include_roles: ::std::option::Option<super::model::SnowflakeListValue>,
    #[prost(message, optional, tag = "3")]
    pub compute_prune_count: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginGuildPruneResponse {
    #[prost(oneof = "begin_guild_prune_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<begin_guild_prune_response::Response>,
}
pub mod begin_guild_prune_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Guild Voice Regions

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildVoiceRegionsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildVoiceRegionsResponse {
    #[prost(oneof = "get_guild_voice_regions_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_voice_regions_response::Response>,
}
pub mod get_guild_voice_regions_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(string, repeated, tag = "1")]
        pub regions: ::std::vec::Vec<std::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Get Guild Invites

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildInvitesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildInvitesResponse {
    #[prost(oneof = "get_guild_invites_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_invites_response::Response>,
}
pub mod get_guild_invites_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub invites: ::std::vec::Vec<super::super::model::InviteData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Channel

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyChannelRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(message, optional, tag = "2")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(enumeration = "super::model::channel_data::ChannelType", tag = "3")]
    pub r#type: i32,
    #[prost(message, optional, tag = "4")]
    pub position: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub topic: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "6")]
    pub nsfw: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "7")]
    pub rate_limit_per_user: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub bitrate: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "9")]
    pub user_limit: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "10")]
    pub permission_overwrites:
        ::std::option::Option<modify_channel_request::ChannelPermissionOverwritesValue>,
    #[prost(message, optional, tag = "11")]
    pub parent_id: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
pub mod modify_channel_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelPermissionOverwritesValue {
        #[prost(message, repeated, tag = "1")]
        pub values:
            ::std::vec::Vec<super::super::model::channel_data::ChannelPermissionOverwriteData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyChannelResponse {
    #[prost(oneof = "modify_channel_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_channel_response::Response>,
}
pub mod modify_channel_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub channel: ::std::option::Option<super::super::model::ChannelData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Delete Channel

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelResponse {
    #[prost(oneof = "delete_channel_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_channel_response::Response>,
}
pub mod delete_channel_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Create Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(string, tag = "2")]
    pub content: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub nonce: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub tts: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub embed: ::std::option::Option<super::model::message_data::MessageEmbedData>,
    #[prost(message, optional, tag = "6")]
    pub allowed_mentions: ::std::option::Option<create_message_request::AllowedMentions>,
    #[prost(message, optional, tag = "7")]
    pub attachment: ::std::option::Option<create_message_request::Attachment>,
}
pub mod create_message_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowedMentions {
        #[prost(message, optional, tag = "1")]
        pub parse: ::std::option::Option<allowed_mentions::AllowedMentionTypes>,
        #[prost(message, optional, tag = "2")]
        pub roles: ::std::option::Option<super::super::model::SnowflakeListValue>,
        #[prost(message, optional, tag = "3")]
        pub users: ::std::option::Option<super::super::model::SnowflakeListValue>,
    }
    pub mod allowed_mentions {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllowedMentionTypes {
            #[prost(bool, tag = "1")]
            pub roles: bool,
            #[prost(bool, tag = "2")]
            pub users: bool,
            #[prost(bool, tag = "3")]
            pub everyone: bool,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attachment {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(bytes, tag = "2")]
        pub content: std::vec::Vec<u8>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMessageResponse {
    #[prost(oneof = "create_message_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_message_response::Response>,
}
pub mod create_message_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub message: ::std::option::Option<super::super::model::MessageData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Crosspost Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrosspostMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrosspostMessageResponse {
    #[prost(oneof = "crosspost_message_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<crosspost_message_response::Response>,
}
pub mod crosspost_message_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub message: ::std::option::Option<super::super::model::MessageData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Create Reaction

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReactionRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(string, tag = "3")]
    pub emoji: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReactionResponse {
    #[prost(oneof = "create_reaction_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_reaction_response::Response>,
}
pub mod create_reaction_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Delete Own Reaction

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOwnReactionRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(string, tag = "3")]
    pub emoji: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOwnReactionResponse {
    #[prost(oneof = "delete_own_reaction_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_own_reaction_response::Response>,
}
pub mod delete_own_reaction_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Delete User Reaction

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserReactionRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(string, tag = "3")]
    pub emoji: std::string::String,
    #[prost(fixed64, tag = "4")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserReactionResponse {
    #[prost(oneof = "delete_user_reaction_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_user_reaction_response::Response>,
}
pub mod delete_user_reaction_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Delete User Reaction

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllReactionsRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllReactionsResponse {
    #[prost(oneof = "delete_all_reactions_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_all_reactions_response::Response>,
}
pub mod delete_all_reactions_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Delete All Reactions for Emoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllReactionsForEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(string, tag = "3")]
    pub emoji: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllReactionsForEmojiResponse {
    #[prost(
        oneof = "delete_all_reactions_for_emoji_response::Response",
        tags = "1, 2"
    )]
    pub response: ::std::option::Option<delete_all_reactions_for_emoji_response::Response>,
}
pub mod delete_all_reactions_for_emoji_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Edit Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(message, optional, tag = "3")]
    pub content: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub embed: ::std::option::Option<super::model::message_data::MessageEmbedData>,
    #[prost(message, optional, tag = "5")]
    pub flags: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditMessageResponse {
    #[prost(oneof = "edit_message_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<edit_message_response::Response>,
}
pub mod edit_message_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub message: ::std::option::Option<super::super::model::MessageData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Delete Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageResponse {
    #[prost(oneof = "delete_message_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_message_response::Response>,
}
pub mod delete_message_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Bulk Delete Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDeleteMessagesRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, repeated, packed = "false", tag = "2")]
    pub message_ids: ::std::vec::Vec<u64>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDeleteMessagesResponse {
    #[prost(oneof = "bulk_delete_messages_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<bulk_delete_messages_response::Response>,
}
pub mod bulk_delete_messages_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Edit Channel Permissions

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditChannelPermissionsRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub overwrite_id: u64,
    #[prost(uint64, tag = "3")]
    pub allow: u64,
    #[prost(uint64, tag = "4")]
    pub deny: u64,
    #[prost(
        enumeration = "super::model::channel_data::channel_permission_overwrite_data::ChannelPermissionOverwriteType",
        tag = "5"
    )]
    pub r#type: i32,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditChannelPermissionsResponse {
    #[prost(oneof = "edit_channel_permissions_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<edit_channel_permissions_response::Response>,
}
pub mod edit_channel_permissions_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Channel Invites

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelInvitesRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelInvitesResponse {
    #[prost(oneof = "get_channel_invites_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_channel_invites_response::Response>,
}
pub mod get_channel_invites_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub invites: ::std::vec::Vec<super::super::model::InviteData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Create Channel Invite

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelInviteRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(message, optional, tag = "2")]
    pub max_age: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "3")]
    pub max_uses: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub temporary: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub unique: ::std::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub target_user: ::std::option::Option<super::model::SnowflakeValue>,
    #[prost(
        enumeration = "super::model::invite_data::InviteTargetUserType",
        tag = "7"
    )]
    pub target_user_type: i32,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelInviteResponse {
    #[prost(oneof = "create_channel_invite_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_channel_invite_response::Response>,
}
pub mod create_channel_invite_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub invite: ::std::option::Option<super::super::model::InviteData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Delete Channel Permission

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelPermissionRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub overwrite_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelPermissionResponse {
    #[prost(oneof = "delete_channel_permission_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_channel_permission_response::Response>,
}
pub mod delete_channel_permission_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Follow News Channel

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNewsChannelRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub webhook_channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNewsChannelResponse {
    #[prost(oneof = "follow_news_channel_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<follow_news_channel_response::Response>,
}
pub mod follow_news_channel_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(fixed64, tag = "1")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "2")]
        pub webhook_id: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Trigger Typing Indicator

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerTypingIndicatorRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerTypingIndicatorResponse {
    #[prost(oneof = "trigger_typing_indicator_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<trigger_typing_indicator_response::Response>,
}
pub mod trigger_typing_indicator_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Pinned Messages

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinnedMessagesRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPinnedMessagesResponse {
    #[prost(oneof = "get_pinned_messages_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_pinned_messages_response::Response>,
}
pub mod get_pinned_messages_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub messages: ::std::vec::Vec<super::super::model::MessageData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Add Pinned Channel Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPinnedChannelMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPinnedChannelMessageResponse {
    #[prost(oneof = "add_pinned_channel_message_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<add_pinned_channel_message_response::Response>,
}
pub mod add_pinned_channel_message_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Delete Pinned Channel Message

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePinnedChannelMessageRequest {
    #[prost(fixed64, tag = "1")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "2")]
    pub message_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePinnedChannelMessageResponse {
    #[prost(
        oneof = "delete_pinned_channel_message_response::Response",
        tags = "1, 2"
    )]
    pub response: ::std::option::Option<delete_pinned_channel_message_response::Response>,
}
pub mod delete_pinned_channel_message_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// List Guild Emojis

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildEmojisRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildEmojisResponse {
    #[prost(oneof = "list_guild_emojis_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<list_guild_emojis_response::Response>,
}
pub mod list_guild_emojis_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, repeated, tag = "1")]
        pub emojis: ::std::vec::Vec<super::super::model::EmojiData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Get Guild Emoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub emoji_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildEmojiResponse {
    #[prost(oneof = "get_guild_emoji_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_guild_emoji_response::Response>,
}
pub mod get_guild_emoji_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub emoji: ::std::option::Option<super::super::model::EmojiData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Create Guild Emoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildEmojiRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::std::option::Option<::std::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub roles: ::std::option::Option<super::model::SnowflakeListValue>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuildEmojiResponse {
    #[prost(oneof = "create_guild_emoji_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_guild_emoji_response::Response>,
}
pub mod create_guild_emoji_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub emoji: ::std::option::Option<super::super::model::EmojiData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Guild Emoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub emoji_id: u64,
    #[prost(message, optional, tag = "2")]
    pub name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub roles: ::std::option::Option<super::model::SnowflakeListValue>,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyGuildEmojiResponse {
    #[prost(oneof = "modify_guild_emoji_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_guild_emoji_response::Response>,
}
pub mod modify_guild_emoji_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub emoji: ::std::option::Option<super::super::model::EmojiData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Delete Guild Emoji

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuildEmojiRequest {
    #[prost(fixed64, tag = "1")]
    pub emoji_id: u64,
    #[prost(message, optional, tag = "100")]
    pub audit_log_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuildEmojiResponse {
    #[prost(oneof = "delete_guild_emoji_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<delete_guild_emoji_response::Response>,
}
pub mod delete_guild_emoji_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Get Current User

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentUserRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentUserResponse {
    #[prost(oneof = "get_current_user_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_current_user_response::Response>,
}
pub mod get_current_user_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub user: ::std::option::Option<super::super::model::UserData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Get User

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    #[prost(fixed64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    #[prost(oneof = "get_user_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<get_user_response::Response>,
}
pub mod get_user_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub user: ::std::option::Option<super::super::model::UserData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Modify Current User

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyCurrentUserRequest {
    #[prost(message, optional, tag = "1")]
    pub username: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "2")]
    pub avatar: ::std::option::Option<::std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyCurrentUserResponse {
    #[prost(oneof = "modify_current_user_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<modify_current_user_response::Response>,
}
pub mod modify_current_user_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub user: ::std::option::Option<super::super::model::UserData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
// Leave Guild

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveGuildRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveGuildResponse {
    #[prost(oneof = "leave_guild_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<leave_guild_response::Response>,
}
pub mod leave_guild_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(()),
    }
}
// Create DM

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDmRequest {
    #[prost(fixed64, tag = "1")]
    pub recipient_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDmResponse {
    #[prost(oneof = "create_dm_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<create_dm_response::Response>,
}
pub mod create_dm_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(message, optional, tag = "1")]
        pub channel: ::std::option::Option<super::super::model::ChannelData>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Error(super::RestError),
        #[prost(message, tag = "2")]
        Data(Data),
    }
}
