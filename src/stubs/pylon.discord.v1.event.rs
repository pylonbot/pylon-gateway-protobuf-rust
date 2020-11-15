#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScope {
    #[prost(fixed64, tag = "1")]
    pub bot_id: u64,
    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::GuildData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::GuildData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::GuildData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::GuildData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PresenceUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::PresenceData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::PresenceData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMemberAddEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::MemberData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMemberUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::MemberData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::MemberData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMemberRemoveEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::MemberData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::ChannelData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::ChannelData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::ChannelData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::ChannelData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPinsUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<channel_pins_update_event::Payload>,
}
pub mod channel_pins_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "2")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "3")]
        pub last_pin_timestamp: ::std::option::Option<::prost_types::Timestamp>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildRoleCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::RoleData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildRoleUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::RoleData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::RoleData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildRoleDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::RoleData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub message: ::std::option::Option<super::model::MessageData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::MessageData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::MessageData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_delete_event::Payload>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::MessageData>,
}
pub mod message_delete_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "3")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageDeleteBulkEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_delete_bulk_event::Payload>,
}
pub mod message_delete_bulk_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, repeated, packed = "false", tag = "1")]
        pub ids: ::std::vec::Vec<u64>,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "3")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReactionAddEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_reaction_add_event::Payload>,
}
pub mod message_reaction_add_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub user_id: u64,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "3")]
        pub message_id: u64,
        #[prost(fixed64, tag = "4")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "6")]
        pub emoji:
            ::std::option::Option<super::super::model::message_data::MessageReactionEmojiData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReactionRemoveEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_reaction_remove_event::Payload>,
}
pub mod message_reaction_remove_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub user_id: u64,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "3")]
        pub message_id: u64,
        #[prost(fixed64, tag = "4")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "5")]
        pub emoji:
            ::std::option::Option<super::super::model::message_data::MessageReactionEmojiData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReactionRemoveAllEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_reaction_remove_all_event::Payload>,
}
pub mod message_reaction_remove_all_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "2")]
        pub message_id: u64,
        #[prost(fixed64, tag = "3")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReactionRemoveEmojiEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<message_reaction_remove_emoji_event::Payload>,
}
pub mod message_reaction_remove_emoji_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "2")]
        pub message_id: u64,
        #[prost(fixed64, tag = "3")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "4")]
        pub emoji:
            ::std::option::Option<super::super::model::message_data::MessageReactionEmojiData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingStartEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<typing_start_event::Payload>,
}
pub mod typing_start_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "2")]
        pub user_id: u64,
        #[prost(message, optional, tag = "3")]
        pub timestamp: ::std::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "4")]
        pub member: ::std::option::Option<super::super::model::MemberData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceStateUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::VoiceStateData>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<super::model::VoiceStateData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceServerUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<voice_server_update_event::Payload>,
}
pub mod voice_server_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
        #[prost(string, tag = "2")]
        pub token: std::string::String,
        #[prost(string, tag = "3")]
        pub endpoint: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<super::model::InviteData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<invite_delete_event::Payload>,
}
pub mod invite_delete_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(string, tag = "3")]
        pub code: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildBanAddEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<guild_ban_add_event::Payload>,
}
pub mod guild_ban_add_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "2")]
        pub user: ::std::option::Option<super::super::model::UserData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildBanRemoveEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<guild_ban_remove_event::Payload>,
}
pub mod guild_ban_remove_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
        #[prost(message, optional, tag = "2")]
        pub user: ::std::option::Option<super::super::model::UserData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildEmojisUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<guild_emojis_update_event::Payload>,
    #[prost(message, optional, tag = "3")]
    pub previously_cached: ::std::option::Option<guild_emojis_update_event::Payload>,
}
pub mod guild_emojis_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
        #[prost(message, repeated, tag = "2")]
        pub emojis: ::std::vec::Vec<super::super::model::EmojiData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildIntegrationsUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<guild_integrations_update_event::Payload>,
}
pub mod guild_integrations_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhooksUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<webhooks_update_event::Payload>,
}
pub mod webhooks_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<integration_create_event::Payload>,
}
pub mod integration_create_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationUpdateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<integration_update_event::Payload>,
}
pub mod integration_update_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationDeleteEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<integration_delete_event::Payload>,
}
pub mod integration_delete_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(fixed64, tag = "1")]
        pub guild_id: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionCreateEvent {
    #[prost(message, optional, tag = "1")]
    pub scope: ::std::option::Option<EventScope>,
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<interaction_create_event::Payload>,
}
pub mod interaction_create_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payload {
        #[prost(sfixed64, tag = "1")]
        pub guild_id: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionResponse {}
