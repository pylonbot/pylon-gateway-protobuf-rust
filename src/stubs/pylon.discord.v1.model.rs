#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnowflakeValue {
    #[prost(fixed64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnowflakeListValue {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub values: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionsValue {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub icon: ::std::option::Option<::std::string::String>,
    #[prost(string, tag = "4")]
    pub region: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub afk_channel_id: ::std::option::Option<SnowflakeValue>,
    #[prost(fixed64, tag = "6")]
    pub owner_id: u64,
    #[prost(message, optional, tag = "7")]
    pub joined_at: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "8")]
    pub splash: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "9")]
    pub discovery_splash: ::std::option::Option<::std::string::String>,
    #[prost(uint32, tag = "10")]
    pub afk_timeout: u32,
    #[prost(uint32, tag = "11")]
    pub member_count: u32,
    #[prost(uint32, tag = "12")]
    pub verification_level: u32,
    #[prost(uint32, tag = "13")]
    pub default_message_notifications: u32,
    #[prost(uint32, tag = "14")]
    pub explicit_content_filter: u32,
    #[prost(string, repeated, tag = "15")]
    pub features: ::std::vec::Vec<std::string::String>,
    #[prost(uint32, tag = "16")]
    pub mfa_level: u32,
    #[prost(bool, tag = "17")]
    pub widget_enabled: bool,
    #[prost(message, optional, tag = "18")]
    pub widget_channel_id: ::std::option::Option<SnowflakeValue>,
    #[prost(message, optional, tag = "19")]
    pub system_channel_id: ::std::option::Option<SnowflakeValue>,
    #[prost(message, optional, tag = "20")]
    pub vanity_url_code: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "21")]
    pub description: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "22")]
    pub banner: ::std::option::Option<::std::string::String>,
    #[prost(uint32, tag = "23")]
    pub premium_tier: u32,
    #[prost(uint32, tag = "24")]
    pub premium_subscription_count: u32,
    #[prost(bool, tag = "25")]
    pub unavailable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub guild_id: ::std::option::Option<SnowflakeValue>,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub topic: ::std::option::Option<::std::string::String>,
    #[prost(enumeration = "channel_data::ChannelType", tag = "5")]
    pub r#type: i32,
    #[prost(bool, tag = "6")]
    pub nsfw: bool,
    #[prost(uint32, tag = "7")]
    pub position: u32,
    #[prost(uint32, tag = "8")]
    pub bitrate: u32,
    #[prost(uint32, tag = "9")]
    pub user_limit: u32,
    #[prost(message, optional, tag = "10")]
    pub parent_id: ::std::option::Option<SnowflakeValue>,
    #[prost(uint32, tag = "11")]
    pub rate_limit_per_user: u32,
    #[prost(message, repeated, tag = "12")]
    pub permission_overwrites: ::std::vec::Vec<channel_data::ChannelPermissionOverwriteData>,
}
pub mod channel_data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelPermissionOverwriteData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(
            enumeration = "channel_permission_overwrite_data::ChannelPermissionOverwriteType",
            tag = "2"
        )]
        pub r#type: i32,
        #[prost(uint64, tag = "3")]
        pub allow: u64,
        #[prost(uint64, tag = "4")]
        pub deny: u64,
    }
    pub mod channel_permission_overwrite_data {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ChannelPermissionOverwriteType {
            Unknown = 0,
            Role = 1,
            Member = 2,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChannelType {
        Unknown = 0,
        GuildText = 1,
        Dm = 2,
        GuildVoice = 3,
        GroupDm = 4,
        GuildCategory = 5,
        GuildNews = 6,
        GuildStore = 7,
        PublicThread = 8,
        PrivateThread = 9,
        GuildStageVoice = 10,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub username: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[prost(uint32, tag = "4")]
    pub discriminator: u32,
    #[prost(bool, tag = "5")]
    pub bot: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
    #[prost(message, optional, tag = "3")]
    pub user: ::std::option::Option<UserData>,
    #[prost(message, optional, tag = "4")]
    pub nick: ::std::option::Option<::std::string::String>,
    #[prost(fixed64, repeated, packed = "false", tag = "5")]
    pub roles: ::std::vec::Vec<u64>,
    #[prost(message, optional, tag = "6")]
    pub joined_at: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub premium_since: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "8")]
    pub permissions: u64,
    #[prost(message, optional, tag = "9")]
    pub pending: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    #[prost(bool, tag = "4")]
    pub managed: bool,
    #[prost(bool, tag = "5")]
    pub mentionable: bool,
    #[prost(bool, tag = "6")]
    pub hoist: bool,
    #[prost(uint32, tag = "7")]
    pub color: u32,
    #[prost(uint32, tag = "8")]
    pub position: u32,
    #[prost(uint64, tag = "9")]
    pub permissions: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmojiData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    #[prost(bool, tag = "4")]
    pub animated: bool,
    #[prost(fixed64, repeated, packed = "false", tag = "5")]
    pub roles: ::std::vec::Vec<u64>,
    #[prost(bool, tag = "6")]
    pub managed: bool,
    #[prost(bool, tag = "7")]
    pub require_colons: bool,
    #[prost(bool, tag = "8")]
    pub available: bool,
    #[prost(fixed64, tag = "9")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceStateData {
    #[prost(message, optional, tag = "1")]
    pub member: ::std::option::Option<MemberData>,
    #[prost(fixed64, tag = "2")]
    pub guild_id: u64,
    #[prost(message, optional, tag = "3")]
    pub channel_id: ::std::option::Option<SnowflakeValue>,
    #[prost(message, optional, tag = "4")]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[prost(bool, tag = "5")]
    pub self_mute: bool,
    #[prost(bool, tag = "6")]
    pub self_deaf: bool,
    #[prost(bool, tag = "7")]
    pub self_video: bool,
    #[prost(bool, tag = "8")]
    pub self_stream: bool,
    #[prost(bool, tag = "9")]
    pub mute: bool,
    #[prost(bool, tag = "10")]
    pub deaf: bool,
    #[prost(bool, tag = "11")]
    pub suppress: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub channel_id: u64,
    #[prost(message, optional, tag = "3")]
    pub guild_id: ::std::option::Option<SnowflakeValue>,
    #[prost(string, tag = "4")]
    pub content: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub edited_timestamp: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub mention_roles: ::std::option::Option<SnowflakeListValue>,
    #[prost(bool, tag = "8")]
    pub tts: bool,
    #[prost(bool, tag = "9")]
    pub mention_everyone: bool,
    #[prost(message, repeated, tag = "10")]
    pub attachments: ::std::vec::Vec<message_data::MessageAttachmentData>,
    #[prost(message, repeated, tag = "11")]
    pub embeds: ::std::vec::Vec<message_data::MessageEmbedData>,
    #[prost(message, repeated, tag = "12")]
    pub mentions: ::std::vec::Vec<message_data::MessageMentionData>,
    #[prost(message, repeated, tag = "13")]
    pub reactions: ::std::vec::Vec<message_data::MessageReactionData>,
    #[prost(bool, tag = "14")]
    pub pinned: bool,
    #[prost(enumeration = "message_data::MessageType", tag = "15")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "16")]
    pub mention_channels: ::std::vec::Vec<message_data::MessageMentionChannelData>,
    #[prost(uint32, tag = "17")]
    pub flags: u32,
    #[prost(message, optional, tag = "18")]
    pub activity: ::std::option::Option<message_data::MessageActivityData>,
    #[prost(message, optional, tag = "19")]
    pub application: ::std::option::Option<message_data::MessageApplicationData>,
    #[prost(message, optional, tag = "20")]
    pub message_reference: ::std::option::Option<message_data::MessageReferenceData>,
    #[prost(message, optional, tag = "21")]
    pub author: ::std::option::Option<UserData>,
    #[prost(message, optional, tag = "22")]
    pub member: ::std::option::Option<MemberData>,
    #[prost(message, optional, tag = "23")]
    pub webhook_id: ::std::option::Option<SnowflakeValue>,
    #[prost(message, repeated, tag = "24")]
    pub stickers: ::std::vec::Vec<message_data::MessageStickerData>,
    #[prost(message, optional, tag = "25")]
    pub interaction: ::std::option::Option<message_data::MessageInteractionData>,
    #[prost(message, optional, tag = "26")]
    pub thread: ::std::option::Option<ChannelData>,
    #[prost(message, repeated, tag = "27")]
    pub components: ::std::vec::Vec<message_data::MessageComponentData>,
}
pub mod message_data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageAttachmentData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub filename: std::string::String,
        #[prost(uint64, tag = "3")]
        pub size: u64,
        #[prost(string, tag = "4")]
        pub url: std::string::String,
        #[prost(string, tag = "5")]
        pub proxy_url: std::string::String,
        #[prost(uint64, tag = "6")]
        pub width: u64,
        #[prost(uint64, tag = "7")]
        pub height: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageMentionData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub username: std::string::String,
        #[prost(string, tag = "3")]
        pub avatar: std::string::String,
        #[prost(uint32, tag = "4")]
        pub discriminator: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageReactionData {
        #[prost(uint32, tag = "1")]
        pub count: u32,
        #[prost(bool, tag = "2")]
        pub me: bool,
        #[prost(message, optional, tag = "3")]
        pub emoji: ::std::option::Option<MessageReactionEmojiData>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageReactionEmojiData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub name: std::string::String,
        #[prost(bool, tag = "3")]
        pub animated: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageApplicationData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub cover_image: std::string::String,
        #[prost(string, tag = "3")]
        pub description: std::string::String,
        #[prost(string, tag = "4")]
        pub icon: std::string::String,
        #[prost(string, tag = "5")]
        pub name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageReferenceData {
        #[prost(fixed64, tag = "1")]
        pub message_id: u64,
        #[prost(fixed64, tag = "2")]
        pub channel_id: u64,
        #[prost(fixed64, tag = "3")]
        pub guild_id: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageActivityData {
        #[prost(uint32, tag = "1")]
        pub r#type: u32,
        #[prost(string, tag = "2")]
        pub party_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageMentionChannelData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(fixed64, tag = "2")]
        pub guild_id: u64,
        #[prost(enumeration = "super::channel_data::ChannelType", tag = "3")]
        pub r#type: i32,
        #[prost(string, tag = "4")]
        pub name: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageStickerData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(fixed64, tag = "2")]
        pub pack_id: u64,
        #[prost(string, tag = "3")]
        pub name: std::string::String,
        #[prost(string, tag = "4")]
        pub description: std::string::String,
        #[prost(string, tag = "5")]
        pub tags: std::string::String,
        #[prost(string, tag = "6")]
        pub asset: std::string::String,
        #[prost(
            enumeration = "message_sticker_data::MessageStickerFormatType",
            tag = "7"
        )]
        pub format_type: i32,
    }
    pub mod message_sticker_data {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MessageStickerFormatType {
            Unknown = 0,
            Png = 1,
            Apng = 2,
            Lottie = 3,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageInteractionData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(
            enumeration = "message_interaction_data::MessageInteractionType",
            tag = "2"
        )]
        pub r#type: i32,
        #[prost(string, tag = "3")]
        pub name: std::string::String,
        #[prost(message, optional, tag = "4")]
        pub user: ::std::option::Option<super::UserData>,
    }
    pub mod message_interaction_data {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MessageInteractionType {
            Unknown = 0,
            Ping = 1,
            ApplicationCommand = 2,
            MessageComponent = 3,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageComponentData {
        #[prost(
            enumeration = "message_component_data::MessageComponentType",
            tag = "1"
        )]
        pub r#type: i32,
        #[prost(
            enumeration = "message_component_data::MessageComponentButtonStyle",
            tag = "2"
        )]
        pub style: i32,
        #[prost(string, tag = "3")]
        pub label: std::string::String,
        #[prost(message, optional, tag = "4")]
        pub emoji: ::std::option::Option<MessageReactionEmojiData>,
        #[prost(string, tag = "5")]
        pub custom_id: std::string::String,
        #[prost(string, tag = "6")]
        pub url: std::string::String,
        #[prost(bool, tag = "7")]
        pub disabled: bool,
        #[prost(message, repeated, tag = "8")]
        pub components: ::std::vec::Vec<MessageComponentData>,
    }
    pub mod message_component_data {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MessageComponentType {
            Unknown = 0,
            ActionRow = 1,
            Button = 2,
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MessageComponentButtonStyle {
            UnknownButton = 0,
            Primary = 1,
            Secondary = 2,
            Success = 3,
            Danger = 4,
            Link = 5,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageEmbedData {
        #[prost(string, tag = "1")]
        pub title: std::string::String,
        #[prost(enumeration = "message_embed_data::MessageEmbedType", tag = "2")]
        pub r#type: i32,
        #[prost(string, tag = "3")]
        pub description: std::string::String,
        #[prost(string, tag = "4")]
        pub url: std::string::String,
        #[prost(message, optional, tag = "5")]
        pub timestamp: ::std::option::Option<::prost_types::Timestamp>,
        #[prost(uint32, tag = "6")]
        pub color: u32,
        #[prost(message, optional, tag = "7")]
        pub footer: ::std::option::Option<message_embed_data::MessageEmbedFooterData>,
        #[prost(message, optional, tag = "8")]
        pub image: ::std::option::Option<message_embed_data::MessageEmbedImageData>,
        #[prost(message, optional, tag = "9")]
        pub thumbnail: ::std::option::Option<message_embed_data::MessageEmbedThumbnailData>,
        #[prost(message, optional, tag = "10")]
        pub video: ::std::option::Option<message_embed_data::MessageEmbedVideoData>,
        #[prost(message, optional, tag = "11")]
        pub provider: ::std::option::Option<message_embed_data::MessageEmbedProviderData>,
        #[prost(message, optional, tag = "12")]
        pub author: ::std::option::Option<message_embed_data::MessageEmbedAuthorData>,
        #[prost(message, repeated, tag = "13")]
        pub fields: ::std::vec::Vec<message_embed_data::MessageEmbedFieldData>,
    }
    pub mod message_embed_data {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedFooterData {
            #[prost(string, tag = "1")]
            pub text: std::string::String,
            #[prost(string, tag = "2")]
            pub icon_url: std::string::String,
            #[prost(string, tag = "3")]
            pub proxy_icon_url: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedImageData {
            #[prost(string, tag = "1")]
            pub url: std::string::String,
            #[prost(uint32, tag = "2")]
            pub width: u32,
            #[prost(uint32, tag = "3")]
            pub height: u32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedThumbnailData {
            #[prost(string, tag = "1")]
            pub url: std::string::String,
            #[prost(string, tag = "2")]
            pub proxy_url: std::string::String,
            #[prost(uint32, tag = "3")]
            pub width: u32,
            #[prost(uint32, tag = "4")]
            pub height: u32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedVideoData {
            #[prost(string, tag = "1")]
            pub url: std::string::String,
            #[prost(uint32, tag = "2")]
            pub width: u32,
            #[prost(uint32, tag = "3")]
            pub height: u32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedProviderData {
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            #[prost(string, tag = "2")]
            pub url: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedAuthorData {
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            #[prost(string, tag = "2")]
            pub url: std::string::String,
            #[prost(string, tag = "3")]
            pub icon_url: std::string::String,
            #[prost(string, tag = "4")]
            pub proxy_icon_url: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MessageEmbedFieldData {
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            #[prost(string, tag = "2")]
            pub value: std::string::String,
            #[prost(bool, tag = "3")]
            pub inline: bool,
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MessageEmbedType {
            Unknown = 0,
            Rich = 1,
            Image = 2,
            Video = 3,
            Gifv = 4,
            Article = 5,
            Link = 6,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        Unknown = 0,
        Default = 1,
        RecipientAdd = 2,
        RecipientRemove = 3,
        Call = 4,
        ChannelNameChange = 5,
        ChannelIconChange = 6,
        ChannelPinnedMessage = 7,
        GuildMemberJoin = 8,
        UserPremiumGuildSubscription = 9,
        UserPremiumGuildSubscriptionTier1 = 10,
        UserPremiumGuildSubscriptionTier2 = 11,
        UserPremiumGuildSubscriptionTier3 = 12,
        ChannelFollowAdd = 13,
        GuildDiscoveryDisqualified = 15,
        GuildDiscoveryRequalified = 16,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationCommandInteractionData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub resolved: ::std::option::Option<
        application_command_interaction_data::ApplicationCommandInteractionDataResolved,
    >,
    #[prost(message, repeated, tag = "4")]
    pub options: ::std::vec::Vec<
        application_command_interaction_data::ApplicationCommandInteractionDataOption,
    >,
    #[prost(string, tag = "5")]
    pub custom_id: std::string::String,
    #[prost(
        enumeration = "message_data::message_component_data::MessageComponentType",
        tag = "6"
    )]
    pub component_type: i32,
}
pub mod application_command_interaction_data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationCommandInteractionDataResolved {
        #[prost(message, repeated, tag = "1")]
        pub users: ::std::vec::Vec<super::UserData>,
        /// partial! missing user, deaf, mute
        #[prost(message, repeated, tag = "2")]
        pub members: ::std::vec::Vec<super::MemberData>,
        #[prost(message, repeated, tag = "3")]
        pub roles: ::std::vec::Vec<super::RoleData>,
        /// partial! missing id, name, type, permissions
        #[prost(message, repeated, tag = "4")]
        pub channels: ::std::vec::Vec<super::ChannelData>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationCommandInteractionDataOption {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(
            enumeration = "application_command_interaction_data_option::ApplicationCommandOptionType",
            tag = "2"
        )]
        pub r#type: i32,
        /// todo: help it depends on above type I guess?
        #[prost(string, tag = "3")]
        pub value: std::string::String,
        #[prost(message, repeated, tag = "4")]
        pub options: ::std::vec::Vec<ApplicationCommandInteractionDataOption>,
    }
    pub mod application_command_interaction_data_option {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ApplicationCommandOptionType {
            Unknown = 0,
            SubCommand = 1,
            SubCommandGroup = 2,
            String = 3,
            Integer = 4,
            Boolean = 5,
            User = 6,
            Channel = 7,
            Role = 8,
            Mentionable = 9,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PresenceData {
    #[prost(fixed64, tag = "1")]
    pub guild_id: u64,
    #[prost(enumeration = "presence_data::OnlineStatus", tag = "4")]
    pub status: i32,
    #[prost(message, optional, tag = "5")]
    pub client_status: ::std::option::Option<presence_data::PresenceClientStatusData>,
    #[prost(message, repeated, tag = "6")]
    pub activities: ::std::vec::Vec<presence_data::PresenceActivityData>,
    #[prost(oneof = "presence_data::MaybePartialUser", tags = "2, 3")]
    pub maybe_partial_user: ::std::option::Option<presence_data::MaybePartialUser>,
}
pub mod presence_data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PresenceClientStatusData {
        #[prost(string, tag = "1")]
        pub desktop: std::string::String,
        #[prost(string, tag = "2")]
        pub mobile: std::string::String,
        #[prost(string, tag = "3")]
        pub web: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PresenceActivityData {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(enumeration = "presence_activity_data::ActivityType", tag = "2")]
        pub r#type: i32,
        #[prost(string, tag = "3")]
        pub url: std::string::String,
        #[prost(message, optional, tag = "4")]
        pub created_at: ::std::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "5")]
        pub timestamps:
            ::std::option::Option<presence_activity_data::PresenceActivityTimestampsData>,
        #[prost(fixed64, tag = "6")]
        pub application_id: u64,
        #[prost(string, tag = "7")]
        pub details: std::string::String,
        #[prost(string, tag = "8")]
        pub state: std::string::String,
        #[prost(message, optional, tag = "9")]
        pub emoji: ::std::option::Option<presence_activity_data::PresenceActivityEmojiData>,
        #[prost(message, optional, tag = "10")]
        pub party: ::std::option::Option<presence_activity_data::PresenceActivityPartyData>,
        #[prost(message, optional, tag = "11")]
        pub assets: ::std::option::Option<presence_activity_data::PresenceActivityAssetsData>,
        #[prost(message, optional, tag = "12")]
        pub secrets: ::std::option::Option<presence_activity_data::PresenceActivitySecretsData>,
        #[prost(bool, tag = "13")]
        pub instance: bool,
        #[prost(uint32, tag = "14")]
        pub flags: u32,
    }
    pub mod presence_activity_data {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PresenceActivityTimestampsData {
            #[prost(uint64, tag = "1")]
            pub start: u64,
            #[prost(uint64, tag = "2")]
            pub end: u64,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PresenceActivityEmojiData {
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            #[prost(fixed64, tag = "2")]
            pub id: u64,
            #[prost(bool, tag = "3")]
            pub animated: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PresenceActivityPartyData {
            #[prost(string, tag = "1")]
            pub id: std::string::String,
            #[prost(uint64, tag = "2")]
            pub current_size: u64,
            #[prost(uint64, tag = "3")]
            pub max_size: u64,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PresenceActivityAssetsData {
            #[prost(string, tag = "1")]
            pub large_image: std::string::String,
            #[prost(string, tag = "2")]
            pub large_text: std::string::String,
            #[prost(string, tag = "3")]
            pub small_image: std::string::String,
            #[prost(string, tag = "4")]
            pub small_text: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PresenceActivitySecretsData {
            #[prost(string, tag = "1")]
            pub join: std::string::String,
            #[prost(string, tag = "2")]
            pub spectate: std::string::String,
            #[prost(string, tag = "3")]
            pub r#match: std::string::String,
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ActivityType {
            Unknown = 0,
            Default = 1,
            Streaming = 2,
            Listening = 3,
            Watching = 4,
            CustomStatus = 5,
            Competing = 6,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OnlineStatus {
        Unknown = 0,
        Online = 1,
        Idle = 2,
        Dnd = 3,
        Invisible = 4,
        Offline = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MaybePartialUser {
        #[prost(message, tag = "2")]
        User(super::UserData),
        #[prost(fixed64, tag = "3")]
        UserId(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookData {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub channel_id: u64,
    #[prost(fixed64, tag = "3")]
    pub guild_id: u64,
    #[prost(enumeration = "webhook_data::WebhookType", tag = "4")]
    pub r#type: i32,
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    #[prost(string, tag = "6")]
    pub avatar: std::string::String,
    #[prost(string, tag = "7")]
    pub token: std::string::String,
}
pub mod webhook_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebhookType {
        Incoming = 0,
        ChannelFollower = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteData {
    #[prost(string, tag = "1")]
    pub code: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub guild: ::std::option::Option<invite_data::InviteGuildData>,
    #[prost(message, optional, tag = "3")]
    pub channel: ::std::option::Option<invite_data::InviteChannelData>,
    #[prost(message, optional, tag = "4")]
    pub inviter: ::std::option::Option<UserData>,
    #[prost(message, optional, tag = "5")]
    pub target_user: ::std::option::Option<UserData>,
    #[prost(enumeration = "invite_data::InviteTargetUserType", tag = "6")]
    pub target_user_type: i32,
    #[prost(message, optional, tag = "7")]
    pub approximate_presence_count: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub approximate_member_count: ::std::option::Option<u32>,
}
pub mod invite_data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InviteGuildData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub name: std::string::String,
        #[prost(message, optional, tag = "3")]
        pub splash: ::std::option::Option<::std::string::String>,
        #[prost(message, optional, tag = "4")]
        pub banner: ::std::option::Option<::std::string::String>,
        #[prost(message, optional, tag = "5")]
        pub description: ::std::option::Option<::std::string::String>,
        #[prost(message, optional, tag = "6")]
        pub icon: ::std::option::Option<::std::string::String>,
        #[prost(string, repeated, tag = "7")]
        pub features: ::std::vec::Vec<std::string::String>,
        #[prost(uint32, tag = "8")]
        pub verification_level: u32,
        #[prost(message, optional, tag = "9")]
        pub vanity_url_code: ::std::option::Option<::std::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InviteChannelData {
        #[prost(fixed64, tag = "1")]
        pub id: u64,
        #[prost(string, tag = "2")]
        pub name: std::string::String,
        #[prost(enumeration = "super::channel_data::ChannelType", tag = "3")]
        pub r#type: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InviteTargetUserType {
        Unknown = 0,
        Stream = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildBanData {
    #[prost(string, tag = "1")]
    pub reason: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub user: ::std::option::Option<UserData>,
}
