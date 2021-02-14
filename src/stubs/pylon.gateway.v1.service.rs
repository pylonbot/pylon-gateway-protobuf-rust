#[doc = r" Generated client implementations."]
pub mod gateway_dispatch_streaming_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GatewayDispatchStreamingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayDispatchStreamingClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayDispatchStreamingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn event(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::discord::v1::event::EventEnvelope,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::discord::v1::event::EventEnvelopeAck,
                >,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatchStreaming/Event",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for GatewayDispatchStreamingClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GatewayDispatchStreamingClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GatewayDispatchStreamingClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod gateway_dispatch_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GatewayDispatchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayDispatchClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayDispatchClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn guild_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn presence_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::PresenceUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/PresenceUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_member_add(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildMemberAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberAdd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_member_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildMemberUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_member_remove(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildMemberRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberRemove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn channel_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::ChannelCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn channel_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::ChannelUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn channel_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::ChannelDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn channel_pins_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::ChannelPinsUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelPinsUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_role_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildRoleCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_role_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildRoleUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_role_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildRoleDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_delete_bulk(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageDeleteBulkEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageDeleteBulk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_reaction_add(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageReactionAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionAdd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_reaction_remove(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageReactionRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_reaction_remove_all(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageReactionRemoveAllEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemoveAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn message_reaction_remove_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::MessageReactionRemoveEmojiEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemoveEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn typing_start(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::TypingStartEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/TypingStart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn voice_state_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::VoiceStateUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/VoiceStateUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn voice_server_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::VoiceServerUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/VoiceServerUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn invite_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::InviteCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/InviteCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn invite_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::InviteDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/InviteDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_ban_add(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildBanAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildBanAdd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_ban_remove(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildBanRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildBanRemove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_emojis_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildEmojisUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildEmojisUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn guild_integrations_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::GuildIntegrationsUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/GuildIntegrationsUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn webhooks_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::WebhooksUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/WebhooksUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn integration_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::IntegrationCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn integration_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::IntegrationUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn integration_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::IntegrationDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn interaction_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::event::InteractionCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::InteractionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayDispatch/InteractionCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GatewayDispatchClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GatewayDispatchClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GatewayDispatchClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gateway_dispatch_streaming_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GatewayDispatchStreamingServer."]
    #[async_trait]
    pub trait GatewayDispatchStreaming: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Event method."]
        type EventStream: Stream<
                Item = Result<
                    super::super::super::super::discord::v1::event::EventEnvelopeAck,
                    tonic::Status,
                >,
            > + Send
            + Sync
            + 'static;
        async fn event(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::super::discord::v1::event::EventEnvelope>,
            >,
        ) -> Result<tonic::Response<Self::EventStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GatewayDispatchStreamingServer<T: GatewayDispatchStreaming> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GatewayDispatchStreaming> GatewayDispatchStreamingServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GatewayDispatchStreamingServer<T>
    where
        T: GatewayDispatchStreaming,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pylon.gateway.v1.service.GatewayDispatchStreaming/Event" => {
                    #[allow(non_camel_case_types)]
                    struct EventSvc<T: GatewayDispatchStreaming>(pub Arc<T>);
                    impl<T: GatewayDispatchStreaming>
                        tonic::server::StreamingService<
                            super::super::super::super::discord::v1::event::EventEnvelope,
                        > for EventSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventEnvelopeAck;
                        type ResponseStream = T::EventStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::discord::v1::event::EventEnvelope,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = EventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GatewayDispatchStreaming> Clone for GatewayDispatchStreamingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GatewayDispatchStreaming> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayDispatchStreaming> tonic::transport::NamedService
        for GatewayDispatchStreamingServer<T>
    {
        const NAME: &'static str = "pylon.gateway.v1.service.GatewayDispatchStreaming";
    }
}
#[doc = r" Generated server implementations."]
pub mod gateway_dispatch_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GatewayDispatchServer."]
    #[async_trait]
    pub trait GatewayDispatch: Send + Sync + 'static {
        async fn guild_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn presence_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::PresenceUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_member_add(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildMemberAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_member_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildMemberUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_member_remove(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildMemberRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn channel_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::ChannelCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn channel_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::ChannelUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn channel_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::ChannelDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn channel_pins_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::ChannelPinsUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_role_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildRoleCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_role_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildRoleUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_role_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildRoleDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_delete_bulk(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageDeleteBulkEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_reaction_add(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageReactionAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_reaction_remove(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageReactionRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_reaction_remove_all(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageReactionRemoveAllEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn message_reaction_remove_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::MessageReactionRemoveEmojiEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn typing_start(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::TypingStartEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn voice_state_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::VoiceStateUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn voice_server_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::VoiceServerUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn invite_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::InviteCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn invite_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::InviteDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_ban_add(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildBanAddEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_ban_remove(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildBanRemoveEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_emojis_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildEmojisUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn guild_integrations_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::GuildIntegrationsUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn webhooks_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::WebhooksUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn integration_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::IntegrationCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn integration_update(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::IntegrationUpdateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn integration_delete(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::IntegrationDeleteEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::EventResponse>,
            tonic::Status,
        >;
        async fn interaction_create(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::event::InteractionCreateEvent,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::event::InteractionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayDispatchServer<T: GatewayDispatch> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GatewayDispatch> GatewayDispatchServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GatewayDispatchServer<T>
    where
        T: GatewayDispatch,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pylon.gateway.v1.service.GatewayDispatch/GuildCreate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildCreateEvent,
                        > for GuildCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildCreateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildUpdateEvent,
                        > for GuildUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildUpdateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildDelete" => {
                    #[allow(non_camel_case_types)]
                    struct GuildDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildDeleteEvent,
                        > for GuildDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildDeleteEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/PresenceUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct PresenceUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::PresenceUpdateEvent,
                        > for PresenceUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::PresenceUpdateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).presence_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PresenceUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberAdd" => {
                    #[allow(non_camel_case_types)]
                    struct GuildMemberAddSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildMemberAddEvent,
                        > for GuildMemberAddSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildMemberAddEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_member_add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildMemberAddSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildMemberUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildMemberUpdateEvent,
                        > for GuildMemberUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildMemberUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_member_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildMemberUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildMemberRemove" => {
                    #[allow(non_camel_case_types)]
                    struct GuildMemberRemoveSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildMemberRemoveEvent,
                        > for GuildMemberRemoveSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildMemberRemoveEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_member_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildMemberRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelCreate" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::ChannelCreateEvent,
                        > for ChannelCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::ChannelCreateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ChannelCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::ChannelUpdateEvent,
                        > for ChannelUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::ChannelUpdateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ChannelUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelDelete" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::ChannelDeleteEvent,
                        > for ChannelDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::ChannelDeleteEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ChannelDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/ChannelPinsUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelPinsUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::ChannelPinsUpdateEvent,
                        > for ChannelPinsUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: ChannelPinsUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_pins_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ChannelPinsUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleCreate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildRoleCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildRoleCreateEvent,
                        > for GuildRoleCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildRoleCreateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_role_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildRoleCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildRoleUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildRoleUpdateEvent,
                        > for GuildRoleUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildRoleUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_role_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildRoleUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildRoleDelete" => {
                    #[allow(non_camel_case_types)]
                    struct GuildRoleDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildRoleDeleteEvent,
                        > for GuildRoleDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildRoleDeleteEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_role_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildRoleDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageCreate" => {
                    #[allow(non_camel_case_types)]
                    struct MessageCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::MessageCreateEvent,
                        > for MessageCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::MessageCreateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct MessageUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::MessageUpdateEvent,
                        > for MessageUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::MessageUpdateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageDelete" => {
                    #[allow(non_camel_case_types)]
                    struct MessageDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::MessageDeleteEvent,
                        > for MessageDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::MessageDeleteEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageDeleteBulk" => {
                    #[allow(non_camel_case_types)]
                    struct MessageDeleteBulkSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::MessageDeleteBulkEvent,
                        > for MessageDeleteBulkSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: MessageDeleteBulkEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message_delete_bulk(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageDeleteBulkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionAdd" => {
                    #[allow(non_camel_case_types)]
                    struct MessageReactionAddSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::MessageReactionAddEvent,
                        > for MessageReactionAddSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionAddEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).message_reaction_add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageReactionAddSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemove" => {
                    #[allow(non_camel_case_types)]
                    struct MessageReactionRemoveSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl < T : GatewayDispatch > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveEvent > for MessageReactionRemoveSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: event :: EventResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveEvent >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . message_reaction_remove (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageReactionRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemoveAll" => {
                    #[allow(non_camel_case_types)]
                    struct MessageReactionRemoveAllSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl < T : GatewayDispatch > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveAllEvent > for MessageReactionRemoveAllSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: event :: EventResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveAllEvent >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . message_reaction_remove_all (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageReactionRemoveAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/MessageReactionRemoveEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct MessageReactionRemoveEmojiSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl < T : GatewayDispatch > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveEmojiEvent > for MessageReactionRemoveEmojiSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: event :: EventResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: MessageReactionRemoveEmojiEvent >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . message_reaction_remove_emoji (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MessageReactionRemoveEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/TypingStart" => {
                    #[allow(non_camel_case_types)]
                    struct TypingStartSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::TypingStartEvent,
                        > for TypingStartSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::TypingStartEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).typing_start(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TypingStartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/VoiceStateUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct VoiceStateUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::VoiceStateUpdateEvent,
                        > for VoiceStateUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: VoiceStateUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).voice_state_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = VoiceStateUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/VoiceServerUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct VoiceServerUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::VoiceServerUpdateEvent,
                        > for VoiceServerUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: VoiceServerUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).voice_server_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = VoiceServerUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/InviteCreate" => {
                    #[allow(non_camel_case_types)]
                    struct InviteCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::InviteCreateEvent,
                        > for InviteCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::InviteCreateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).invite_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = InviteCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/InviteDelete" => {
                    #[allow(non_camel_case_types)]
                    struct InviteDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::InviteDeleteEvent,
                        > for InviteDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::InviteDeleteEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).invite_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = InviteDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildBanAdd" => {
                    #[allow(non_camel_case_types)]
                    struct GuildBanAddSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildBanAddEvent,
                        > for GuildBanAddSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildBanAddEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_ban_add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildBanAddSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildBanRemove" => {
                    #[allow(non_camel_case_types)]
                    struct GuildBanRemoveSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildBanRemoveEvent,
                        > for GuildBanRemoveSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::GuildBanRemoveEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_ban_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildBanRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildEmojisUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildEmojisUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::GuildEmojisUpdateEvent,
                        > for GuildEmojisUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildEmojisUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).guild_emojis_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildEmojisUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/GuildIntegrationsUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct GuildIntegrationsUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl < T : GatewayDispatch > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: event :: GuildIntegrationsUpdateEvent > for GuildIntegrationsUpdateSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: event :: EventResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: GuildIntegrationsUpdateEvent >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . guild_integrations_update (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GuildIntegrationsUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/WebhooksUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct WebhooksUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::WebhooksUpdateEvent,
                        > for WebhooksUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::event::WebhooksUpdateEvent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).webhooks_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = WebhooksUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationCreate" => {
                    #[allow(non_camel_case_types)]
                    struct IntegrationCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::IntegrationCreateEvent,
                        > for IntegrationCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: IntegrationCreateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).integration_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = IntegrationCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct IntegrationUpdateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::IntegrationUpdateEvent,
                        > for IntegrationUpdateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: IntegrationUpdateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).integration_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = IntegrationUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/IntegrationDelete" => {
                    #[allow(non_camel_case_types)]
                    struct IntegrationDeleteSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::IntegrationDeleteEvent,
                        > for IntegrationDeleteSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::EventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: IntegrationDeleteEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).integration_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = IntegrationDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayDispatch/InteractionCreate" => {
                    #[allow(non_camel_case_types)]
                    struct InteractionCreateSvc<T: GatewayDispatch>(pub Arc<T>);
                    impl<T: GatewayDispatch>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::event::InteractionCreateEvent,
                        > for InteractionCreateSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::event::InteractionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: event :: InteractionCreateEvent >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).interaction_create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = InteractionCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GatewayDispatch> Clone for GatewayDispatchServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GatewayDispatch> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayDispatch> tonic::transport::NamedService for GatewayDispatchServer<T> {
        const NAME: &'static str = "pylon.gateway.v1.service.GatewayDispatch";
    }
}
#[doc = r" Generated client implementations."]
pub mod gateway_cache_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GatewayCacheClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayCacheClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayCacheClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Guilds"]
        pub async fn get_guild(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Channels"]
        pub async fn list_guild_channels(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::ListGuildChannelsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildChannelsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/ListGuildChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_channel(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildChannelResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Guild Members"]
        pub async fn list_guild_members(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::ListGuildMembersRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildMembersResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/ListGuildMembers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_member(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildMemberResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMember",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_guild_members(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::FindGuildMembersRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::FindGuildMembersResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/FindGuildMembers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Guild Member Presence"]
        pub async fn get_guild_member_presence(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildMemberPresenceRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildMemberPresenceResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMemberPresence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Guild Member Roles"]
        pub async fn list_guild_roles(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::ListGuildRolesRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::ListGuildRolesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/ListGuildRoles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildRoleResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Emojis"]
        pub async fn list_guild_emojis(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::ListGuildEmojisRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildEmojisResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/ListGuildEmojis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildEmojiResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " VoiceStates"]
        pub async fn get_guild_member_voice_state(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetGuildMemberVoiceStateRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildMemberVoiceStateResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMemberVoiceState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_guild_channel_voice_states(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::ListGuildChannelVoiceStatesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildChannelVoiceStatesResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/ListGuildChannelVoiceStates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetUser"]
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::cache::GetUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayCache/GetUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GatewayCacheClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GatewayCacheClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GatewayCacheClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gateway_cache_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GatewayCacheServer."]
    #[async_trait]
    pub trait GatewayCache: Send + Sync + 'static {
        #[doc = " Guilds"]
        async fn get_guild(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildResponse>,
            tonic::Status,
        >;
        #[doc = " Channels"]
        async fn list_guild_channels(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::ListGuildChannelsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildChannelsResponse,
            >,
            tonic::Status,
        >;
        async fn get_guild_channel(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildChannelResponse,
            >,
            tonic::Status,
        >;
        #[doc = " Guild Members"]
        async fn list_guild_members(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::ListGuildMembersRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildMembersResponse,
            >,
            tonic::Status,
        >;
        async fn get_guild_member(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildMemberResponse>,
            tonic::Status,
        >;
        async fn find_guild_members(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::FindGuildMembersRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::FindGuildMembersResponse,
            >,
            tonic::Status,
        >;
        #[doc = " Guild Member Presence"]
        async fn get_guild_member_presence(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildMemberPresenceRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildMemberPresenceResponse,
            >,
            tonic::Status,
        >;
        #[doc = " Guild Member Roles"]
        async fn list_guild_roles(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::ListGuildRolesRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::ListGuildRolesResponse>,
            tonic::Status,
        >;
        async fn get_guild_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildRoleResponse>,
            tonic::Status,
        >;
        #[doc = " Emojis"]
        async fn list_guild_emojis(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::ListGuildEmojisRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildEmojisResponse,
            >,
            tonic::Status,
        >;
        async fn get_guild_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetGuildEmojiResponse>,
            tonic::Status,
        >;
        #[doc = " VoiceStates"]
        async fn get_guild_member_voice_state(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::GetGuildMemberVoiceStateRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::GetGuildMemberVoiceStateResponse,
            >,
            tonic::Status,
        >;
        async fn list_guild_channel_voice_states(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::cache::ListGuildChannelVoiceStatesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::cache::ListGuildChannelVoiceStatesResponse,
            >,
            tonic::Status,
        >;
        #[doc = " GetUser"]
        async fn get_user(
            &self,
            request: tonic::Request<super::super::super::super::discord::v1::cache::GetUserRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::cache::GetUserResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayCacheServer<T: GatewayCache> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GatewayCache> GatewayCacheServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GatewayCacheServer<T>
    where
        T: GatewayCache,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pylon.gateway.v1.service.GatewayCache/GetGuild" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetGuildRequest,
                        > for GetGuildSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetGuildResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::cache::GetGuildRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/ListGuildChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildChannelsSvc<T: GatewayCache>(pub Arc<T>);
                    impl < T : GatewayCache > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelsRequest > for ListGuildChannelsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . list_guild_channels (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildChannelsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildChannel" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildChannelSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetGuildChannelRequest,
                        > for GetGuildChannelSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetGuildChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildChannelRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/ListGuildMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildMembersSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::ListGuildMembersRequest,
                        > for ListGuildMembersSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildMembersResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildMembersRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_guild_members(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildMembersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMember" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildMemberSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetGuildMemberRequest,
                        > for GetGuildMemberSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetGuildMemberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_member(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildMemberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/FindGuildMembers" => {
                    #[allow(non_camel_case_types)]
                    struct FindGuildMembersSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::FindGuildMembersRequest,
                        > for FindGuildMembersSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: cache :: FindGuildMembersResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: FindGuildMembersRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_guild_members(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindGuildMembersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMemberPresence" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildMemberPresenceSvc<T: GatewayCache>(pub Arc<T>);
                    impl < T : GatewayCache > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberPresenceRequest > for GetGuildMemberPresenceSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberPresenceResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberPresenceRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_guild_member_presence (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildMemberPresenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/ListGuildRoles" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildRolesSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::ListGuildRolesRequest,
                        > for ListGuildRolesSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::ListGuildRolesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildRolesRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_guild_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildRolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildRole" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildRoleSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetGuildRoleRequest,
                        > for GetGuildRoleSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetGuildRoleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::cache::GetGuildRoleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/ListGuildEmojis" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildEmojisSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::ListGuildEmojisRequest,
                        > for ListGuildEmojisSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::ListGuildEmojisResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildEmojisRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_guild_emojis(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildEmojisSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildEmojiSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetGuildEmojiRequest,
                        > for GetGuildEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetGuildEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildEmojiRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetGuildMemberVoiceState" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildMemberVoiceStateSvc<T: GatewayCache>(pub Arc<T>);
                    impl < T : GatewayCache > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberVoiceStateRequest > for GetGuildMemberVoiceStateSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberVoiceStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: GetGuildMemberVoiceStateRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_guild_member_voice_state (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildMemberVoiceStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/ListGuildChannelVoiceStates" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildChannelVoiceStatesSvc<T: GatewayCache>(pub Arc<T>);
                    impl < T : GatewayCache > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelVoiceStatesRequest > for ListGuildChannelVoiceStatesSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelVoiceStatesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: cache :: ListGuildChannelVoiceStatesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . list_guild_channel_voice_states (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildChannelVoiceStatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayCache/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: GatewayCache>(pub Arc<T>);
                    impl<T: GatewayCache>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::cache::GetUserRequest,
                        > for GetUserSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::cache::GetUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::cache::GetUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GatewayCache> Clone for GatewayCacheServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GatewayCache> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayCache> tonic::transport::NamedService for GatewayCacheServer<T> {
        const NAME: &'static str = "pylon.gateway.v1.service.GatewayCache";
    }
}
#[doc = r" Generated client implementations."]
pub mod gateway_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GatewayClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn update_voice_state(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::gateway::UpdateVoiceStateRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::gateway::UpdateVoiceStateResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.Gateway/UpdateVoiceState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::gateway::UpdateStatusRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::UpdateStatusResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.Gateway/UpdateStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::gateway::FindUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::FindUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/pylon.gateway.v1.service.Gateway/FindUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_user_mutual_guilds(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::gateway::GetUserMutualGuildsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::gateway::GetUserMutualGuildsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.Gateway/FindUserMutualGuilds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::gateway::FindEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::FindEmojiResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/pylon.gateway.v1.service.Gateway/FindEmoji");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GatewayClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GatewayClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GatewayClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gateway_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GatewayServer."]
    #[async_trait]
    pub trait Gateway: Send + Sync + 'static {
        async fn update_voice_state(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::gateway::UpdateVoiceStateRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::gateway::UpdateVoiceStateResponse,
            >,
            tonic::Status,
        >;
        async fn update_status(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::gateway::UpdateStatusRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::UpdateStatusResponse>,
            tonic::Status,
        >;
        async fn find_user(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::gateway::FindUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::FindUserResponse>,
            tonic::Status,
        >;
        async fn find_user_mutual_guilds(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::gateway::GetUserMutualGuildsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::gateway::GetUserMutualGuildsResponse,
            >,
            tonic::Status,
        >;
        async fn find_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::gateway::FindEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::gateway::FindEmojiResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayServer<T: Gateway> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Gateway> GatewayServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GatewayServer<T>
    where
        T: Gateway,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pylon.gateway.v1.service.Gateway/UpdateVoiceState" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVoiceStateSvc<T: Gateway>(pub Arc<T>);
                    impl < T : Gateway > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: gateway :: UpdateVoiceStateRequest > for UpdateVoiceStateSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: gateway :: UpdateVoiceStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: gateway :: UpdateVoiceStateRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . update_voice_state (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateVoiceStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.Gateway/UpdateStatus" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateStatusSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::gateway::UpdateStatusRequest,
                        > for UpdateStatusSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::gateway::UpdateStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: gateway :: UpdateStatusRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.Gateway/FindUser" => {
                    #[allow(non_camel_case_types)]
                    struct FindUserSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::gateway::FindUserRequest,
                        > for FindUserSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::gateway::FindUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::gateway::FindUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.Gateway/FindUserMutualGuilds" => {
                    #[allow(non_camel_case_types)]
                    struct FindUserMutualGuildsSvc<T: Gateway>(pub Arc<T>);
                    impl < T : Gateway > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: gateway :: GetUserMutualGuildsRequest > for FindUserMutualGuildsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: gateway :: GetUserMutualGuildsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: gateway :: GetUserMutualGuildsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . find_user_mutual_guilds (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindUserMutualGuildsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.Gateway/FindEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct FindEmojiSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::gateway::FindEmojiRequest,
                        > for FindEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::gateway::FindEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::gateway::FindEmojiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FindEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Gateway> Clone for GatewayServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Gateway> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Gateway> tonic::transport::NamedService for GatewayServer<T> {
        const NAME: &'static str = "pylon.gateway.v1.service.Gateway";
    }
}
#[doc = r" Generated client implementations."]
pub mod gateway_rest_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GatewayRestClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayRestClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayRestClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn modify_guild(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyGuildResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_guild_channel(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateGuildChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateGuildChannelResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_guild_channel_positions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildChannelPositionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildChannelPositionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildChannelPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_guild_member(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::AddGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::AddGuildMemberResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/AddGuildMember",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_guild_member(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildMemberResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildMember",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_current_user_nick(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserNickRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserNickResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyCurrentUserNick",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_guild_member_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::AddGuildMemberRoleRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::AddGuildMemberRoleResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/AddGuildMemberRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_guild_member_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRoleRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRoleResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildMemberRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_guild_member(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildMember",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_bans(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildBansRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildBansResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildBans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_ban(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildBanResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildBan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_guild_ban(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateGuildBanResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildBan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_guild_ban(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::RemoveGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::RemoveGuildBanResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildBan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_guild_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateGuildRoleResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_guild_role_positions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildRolePositionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildRolePositionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildRolePositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_guild_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyGuildRoleResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_guild_role(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteGuildRoleResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteGuildRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_prune_count(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildPruneCountRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetGuildPruneCountResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildPruneCount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn begin_guild_prune(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::BeginGuildPruneRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::BeginGuildPruneResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/BeginGuildPrune",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_voice_regions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildVoiceRegionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetGuildVoiceRegionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildVoiceRegions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_invites(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildInvitesRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildInvitesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildInvites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_channel(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyChannelRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyChannelResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_channel(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteChannelRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteChannelResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_channel_messages(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetChannelMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelMessagesResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetChannelMessages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_channel_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelMessageResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetChannelMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateMessageResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn crosspost_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CrosspostMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CrosspostMessageResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CrosspostMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_reaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateReactionRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateReactionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateReaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_own_reaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteOwnReactionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteOwnReactionResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteOwnReaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_user_reaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteUserReactionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteUserReactionResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteUserReaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_all_reactions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteAllReactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_all_reactions_for_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsForEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsForEmojiResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteAllReactionsForEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn edit_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::EditMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::EditMessageResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/EditMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteMessageResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn bulk_delete_messages(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::BulkDeleteMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::BulkDeleteMessagesResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/BulkDeleteMessages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn edit_channel_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::EditChannelPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::EditChannelPermissionsResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/EditChannelPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_channel_invites(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetChannelInvitesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelInvitesResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetChannelInvites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_channel_invite(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateChannelInviteRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateChannelInviteResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateChannelInvite",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_channel_permission(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteChannelPermissionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteChannelPermissionResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteChannelPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn follow_news_channel(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::FollowNewsChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::FollowNewsChannelResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/FollowNewsChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn trigger_typing_indicator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::TriggerTypingIndicatorRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::TriggerTypingIndicatorResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/TriggerTypingIndicator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pinned_messages(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetPinnedMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetPinnedMessagesResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetPinnedMessages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_pinned_channel_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::AddPinnedChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::AddPinnedChannelMessageResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/AddPinnedChannelMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_pinned_channel_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeletePinnedChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeletePinnedChannelMessageResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeletePinnedChannelMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_guild_emojis(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ListGuildEmojisRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ListGuildEmojisResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ListGuildEmojis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_guild_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildEmojiResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetGuildEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_guild_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateGuildEmojiResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_guild_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildEmojiResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_guild_emoji(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::DeleteGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteGuildEmojiResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/DeleteGuildEmoji",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_current_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetCurrentUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetCurrentUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetCurrentUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::GetUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/GetUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn modify_current_user(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserResponse,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/ModifyCurrentUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn leave_guild(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::LeaveGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::LeaveGuildResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/LeaveGuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_dm(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::discord::v1::rest::CreateDmRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateDmResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pylon.gateway.v1.service.GatewayRest/CreateDm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GatewayRestClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GatewayRestClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GatewayRestClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gateway_rest_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GatewayRestServer."]
    #[async_trait]
    pub trait GatewayRest: Send + Sync + 'static {
        async fn modify_guild(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyGuildResponse>,
            tonic::Status,
        >;
        async fn create_guild_channel(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateGuildChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateGuildChannelResponse,
            >,
            tonic::Status,
        >;
        async fn modify_guild_channel_positions(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildChannelPositionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildChannelPositionsResponse,
            >,
            tonic::Status,
        >;
        async fn add_guild_member(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::AddGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::AddGuildMemberResponse>,
            tonic::Status,
        >;
        async fn modify_guild_member(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildMemberResponse,
            >,
            tonic::Status,
        >;
        async fn modify_current_user_nick(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserNickRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserNickResponse,
            >,
            tonic::Status,
        >;
        async fn add_guild_member_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::AddGuildMemberRoleRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::AddGuildMemberRoleResponse,
            >,
            tonic::Status,
        >;
        async fn remove_guild_member_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRoleRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRoleResponse,
            >,
            tonic::Status,
        >;
        async fn remove_guild_member(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::RemoveGuildMemberResponse,
            >,
            tonic::Status,
        >;
        async fn get_guild_bans(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildBansRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildBansResponse>,
            tonic::Status,
        >;
        async fn get_guild_ban(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildBanResponse>,
            tonic::Status,
        >;
        async fn create_guild_ban(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateGuildBanResponse>,
            tonic::Status,
        >;
        async fn remove_guild_ban(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::RemoveGuildBanRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::RemoveGuildBanResponse>,
            tonic::Status,
        >;
        async fn create_guild_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateGuildRoleResponse>,
            tonic::Status,
        >;
        async fn modify_guild_role_positions(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildRolePositionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildRolePositionsResponse,
            >,
            tonic::Status,
        >;
        async fn modify_guild_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyGuildRoleResponse>,
            tonic::Status,
        >;
        async fn delete_guild_role(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteGuildRoleRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteGuildRoleResponse>,
            tonic::Status,
        >;
        async fn get_guild_prune_count(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildPruneCountRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetGuildPruneCountResponse,
            >,
            tonic::Status,
        >;
        async fn begin_guild_prune(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::BeginGuildPruneRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::BeginGuildPruneResponse>,
            tonic::Status,
        >;
        async fn get_guild_voice_regions(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildVoiceRegionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetGuildVoiceRegionsResponse,
            >,
            tonic::Status,
        >;
        async fn get_guild_invites(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildInvitesRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildInvitesResponse>,
            tonic::Status,
        >;
        async fn modify_channel(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyChannelRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ModifyChannelResponse>,
            tonic::Status,
        >;
        async fn delete_channel(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteChannelRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteChannelResponse>,
            tonic::Status,
        >;
        async fn get_channel_messages(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetChannelMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelMessagesResponse,
            >,
            tonic::Status,
        >;
        async fn get_channel_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelMessageResponse,
            >,
            tonic::Status,
        >;
        async fn create_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateMessageResponse>,
            tonic::Status,
        >;
        async fn crosspost_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CrosspostMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CrosspostMessageResponse,
            >,
            tonic::Status,
        >;
        async fn create_reaction(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateReactionRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateReactionResponse>,
            tonic::Status,
        >;
        async fn delete_own_reaction(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteOwnReactionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteOwnReactionResponse,
            >,
            tonic::Status,
        >;
        async fn delete_user_reaction(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteUserReactionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteUserReactionResponse,
            >,
            tonic::Status,
        >;
        async fn delete_all_reactions(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsResponse,
            >,
            tonic::Status,
        >;
        async fn delete_all_reactions_for_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsForEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteAllReactionsForEmojiResponse,
            >,
            tonic::Status,
        >;
        async fn edit_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::EditMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::EditMessageResponse>,
            tonic::Status,
        >;
        async fn delete_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteMessageRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::DeleteMessageResponse>,
            tonic::Status,
        >;
        async fn bulk_delete_messages(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::BulkDeleteMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::BulkDeleteMessagesResponse,
            >,
            tonic::Status,
        >;
        async fn edit_channel_permissions(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::EditChannelPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::EditChannelPermissionsResponse,
            >,
            tonic::Status,
        >;
        async fn get_channel_invites(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetChannelInvitesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetChannelInvitesResponse,
            >,
            tonic::Status,
        >;
        async fn create_channel_invite(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateChannelInviteRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateChannelInviteResponse,
            >,
            tonic::Status,
        >;
        async fn delete_channel_permission(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteChannelPermissionRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteChannelPermissionResponse,
            >,
            tonic::Status,
        >;
        async fn follow_news_channel(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::FollowNewsChannelRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::FollowNewsChannelResponse,
            >,
            tonic::Status,
        >;
        async fn trigger_typing_indicator(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::TriggerTypingIndicatorRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::TriggerTypingIndicatorResponse,
            >,
            tonic::Status,
        >;
        async fn get_pinned_messages(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetPinnedMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::GetPinnedMessagesResponse,
            >,
            tonic::Status,
        >;
        async fn add_pinned_channel_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::AddPinnedChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::AddPinnedChannelMessageResponse,
            >,
            tonic::Status,
        >;
        async fn delete_pinned_channel_message(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeletePinnedChannelMessageRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeletePinnedChannelMessageResponse,
            >,
            tonic::Status,
        >;
        async fn list_guild_emojis(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ListGuildEmojisRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::ListGuildEmojisResponse>,
            tonic::Status,
        >;
        async fn get_guild_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetGuildEmojiResponse>,
            tonic::Status,
        >;
        async fn create_guild_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::CreateGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::CreateGuildEmojiResponse,
            >,
            tonic::Status,
        >;
        async fn modify_guild_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyGuildEmojiResponse,
            >,
            tonic::Status,
        >;
        async fn delete_guild_emoji(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::DeleteGuildEmojiRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::DeleteGuildEmojiResponse,
            >,
            tonic::Status,
        >;
        async fn get_current_user(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::GetCurrentUserRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetCurrentUserResponse>,
            tonic::Status,
        >;
        async fn get_user(
            &self,
            request: tonic::Request<super::super::super::super::discord::v1::rest::GetUserRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::GetUserResponse>,
            tonic::Status,
        >;
        async fn modify_current_user(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::discord::v1::rest::ModifyCurrentUserResponse,
            >,
            tonic::Status,
        >;
        async fn leave_guild(
            &self,
            request: tonic::Request<
                super::super::super::super::discord::v1::rest::LeaveGuildRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::LeaveGuildResponse>,
            tonic::Status,
        >;
        async fn create_dm(
            &self,
            request: tonic::Request<super::super::super::super::discord::v1::rest::CreateDmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::discord::v1::rest::CreateDmResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayRestServer<T: GatewayRest> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GatewayRest> GatewayRestServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GatewayRestServer<T>
    where
        T: GatewayRest,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuild" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyGuildRequest,
                        > for ModifyGuildSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::ModifyGuildResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::ModifyGuildRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_guild(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGuildChannelSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildChannelRequest > for CreateGuildChannelSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildChannelResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildChannelRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . create_guild_channel (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGuildChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildChannelPositions" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildChannelPositionsSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildChannelPositionsRequest > for ModifyGuildChannelPositionsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildChannelPositionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildChannelPositionsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . modify_guild_channel_positions (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildChannelPositionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/AddGuildMember" => {
                    #[allow(non_camel_case_types)]
                    struct AddGuildMemberSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::AddGuildMemberRequest,
                        > for AddGuildMemberSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::AddGuildMemberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: AddGuildMemberRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_guild_member(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddGuildMemberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildMember" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildMemberSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyGuildMemberRequest,
                        > for ModifyGuildMemberSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildMemberResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildMemberRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_guild_member(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildMemberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyCurrentUserNick" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyCurrentUserNickSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyCurrentUserNickRequest > for ModifyCurrentUserNickSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: ModifyCurrentUserNickResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyCurrentUserNickRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . modify_current_user_nick (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyCurrentUserNickSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/AddGuildMemberRole" => {
                    #[allow(non_camel_case_types)]
                    struct AddGuildMemberRoleSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: AddGuildMemberRoleRequest > for AddGuildMemberRoleSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: AddGuildMemberRoleResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: AddGuildMemberRoleRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . add_guild_member_role (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddGuildMemberRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildMemberRole" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGuildMemberRoleSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildMemberRoleRequest > for RemoveGuildMemberRoleSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildMemberRoleResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildMemberRoleRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . remove_guild_member_role (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveGuildMemberRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGuildMemberSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::RemoveGuildMemberRequest,
                        > for RemoveGuildMemberSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildMemberResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildMemberRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_guild_member(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveGuildMemberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildBans" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildBansSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetGuildBansRequest,
                        > for GetGuildBansSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetGuildBansResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::GetGuildBansRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_bans(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildBansSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildBan" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildBanSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetGuildBanRequest,
                        > for GetGuildBanSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetGuildBanResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::GetGuildBanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_ban(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildBanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildBan" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGuildBanSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateGuildBanRequest,
                        > for CreateGuildBanSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateGuildBanResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildBanRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_guild_ban(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGuildBanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/RemoveGuildBan" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGuildBanSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::RemoveGuildBanRequest,
                        > for RemoveGuildBanSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::RemoveGuildBanResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: RemoveGuildBanRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_guild_ban(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveGuildBanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildRole" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGuildRoleSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateGuildRoleRequest,
                        > for CreateGuildRoleSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateGuildRoleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildRoleRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_guild_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGuildRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildRolePositions" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildRolePositionsSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildRolePositionsRequest > for ModifyGuildRolePositionsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildRolePositionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildRolePositionsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . modify_guild_role_positions (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildRolePositionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildRole" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildRoleSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyGuildRoleRequest,
                        > for ModifyGuildRoleSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::ModifyGuildRoleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildRoleRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_guild_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteGuildRole" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGuildRoleSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::DeleteGuildRoleRequest,
                        > for DeleteGuildRoleSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::DeleteGuildRoleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteGuildRoleRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_guild_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteGuildRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildPruneCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildPruneCountSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildPruneCountRequest > for GetGuildPruneCountSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildPruneCountResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildPruneCountRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_guild_prune_count (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildPruneCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/BeginGuildPrune" => {
                    #[allow(non_camel_case_types)]
                    struct BeginGuildPruneSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::BeginGuildPruneRequest,
                        > for BeginGuildPruneSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::BeginGuildPruneResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: BeginGuildPruneRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).begin_guild_prune(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BeginGuildPruneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildVoiceRegions" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildVoiceRegionsSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildVoiceRegionsRequest > for GetGuildVoiceRegionsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildVoiceRegionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildVoiceRegionsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_guild_voice_regions (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildVoiceRegionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildInvites" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildInvitesSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetGuildInvitesRequest,
                        > for GetGuildInvitesSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetGuildInvitesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetGuildInvitesRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_invites(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildInvitesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyChannel" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyChannelSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyChannelRequest,
                        > for ModifyChannelSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::ModifyChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::ModifyChannelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteChannel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteChannelSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::DeleteChannelRequest,
                        > for DeleteChannelSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::DeleteChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::DeleteChannelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetChannelMessages" => {
                    #[allow(non_camel_case_types)]
                    struct GetChannelMessagesSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelMessagesRequest > for GetChannelMessagesSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelMessagesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelMessagesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_channel_messages (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetChannelMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetChannelMessage" => {
                    #[allow(non_camel_case_types)]
                    struct GetChannelMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetChannelMessageRequest,
                        > for GetChannelMessageSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelMessageResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelMessageRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_channel_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetChannelMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateMessage" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateMessageRequest,
                        > for CreateMessageSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::CreateMessageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CrosspostMessage" => {
                    #[allow(non_camel_case_types)]
                    struct CrosspostMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CrosspostMessageRequest,
                        > for CrosspostMessageSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CrosspostMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CrosspostMessageRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).crosspost_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CrosspostMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateReaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateReactionSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateReactionRequest,
                        > for CreateReactionSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateReactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateReactionRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_reaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateReactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteOwnReaction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOwnReactionSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::DeleteOwnReactionRequest,
                        > for DeleteOwnReactionSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeleteOwnReactionResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteOwnReactionRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_own_reaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteOwnReactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteUserReaction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserReactionSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteUserReactionRequest > for DeleteUserReactionSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeleteUserReactionResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteUserReactionRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delete_user_reaction (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteUserReactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteAllReactions" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAllReactionsSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsRequest > for DeleteAllReactionsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delete_all_reactions (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAllReactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteAllReactionsForEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAllReactionsForEmojiSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsForEmojiRequest > for DeleteAllReactionsForEmojiSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsForEmojiResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteAllReactionsForEmojiRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delete_all_reactions_for_emoji (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAllReactionsForEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/EditMessage" => {
                    #[allow(non_camel_case_types)]
                    struct EditMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::EditMessageRequest,
                        > for EditMessageSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::EditMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::EditMessageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EditMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteMessage" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::DeleteMessageRequest,
                        > for DeleteMessageSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::DeleteMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::DeleteMessageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/BulkDeleteMessages" => {
                    #[allow(non_camel_case_types)]
                    struct BulkDeleteMessagesSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: BulkDeleteMessagesRequest > for BulkDeleteMessagesSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: BulkDeleteMessagesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: BulkDeleteMessagesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . bulk_delete_messages (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BulkDeleteMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/EditChannelPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct EditChannelPermissionsSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: EditChannelPermissionsRequest > for EditChannelPermissionsSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: EditChannelPermissionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: EditChannelPermissionsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . edit_channel_permissions (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EditChannelPermissionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetChannelInvites" => {
                    #[allow(non_camel_case_types)]
                    struct GetChannelInvitesSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetChannelInvitesRequest,
                        > for GetChannelInvitesSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelInvitesResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetChannelInvitesRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_channel_invites(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetChannelInvitesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateChannelInvite" => {
                    #[allow(non_camel_case_types)]
                    struct CreateChannelInviteSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: CreateChannelInviteRequest > for CreateChannelInviteSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: CreateChannelInviteResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateChannelInviteRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . create_channel_invite (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateChannelInviteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteChannelPermission" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteChannelPermissionSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteChannelPermissionRequest > for DeleteChannelPermissionSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeleteChannelPermissionResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteChannelPermissionRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delete_channel_permission (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteChannelPermissionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/FollowNewsChannel" => {
                    #[allow(non_camel_case_types)]
                    struct FollowNewsChannelSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::FollowNewsChannelRequest,
                        > for FollowNewsChannelSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: FollowNewsChannelResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: FollowNewsChannelRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).follow_news_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FollowNewsChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/TriggerTypingIndicator" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerTypingIndicatorSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: TriggerTypingIndicatorRequest > for TriggerTypingIndicatorSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: TriggerTypingIndicatorResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: TriggerTypingIndicatorRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . trigger_typing_indicator (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TriggerTypingIndicatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetPinnedMessages" => {
                    #[allow(non_camel_case_types)]
                    struct GetPinnedMessagesSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetPinnedMessagesRequest,
                        > for GetPinnedMessagesSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: GetPinnedMessagesResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetPinnedMessagesRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_pinned_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPinnedMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/AddPinnedChannelMessage" => {
                    #[allow(non_camel_case_types)]
                    struct AddPinnedChannelMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: AddPinnedChannelMessageRequest > for AddPinnedChannelMessageSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: AddPinnedChannelMessageResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: AddPinnedChannelMessageRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . add_pinned_channel_message (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddPinnedChannelMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeletePinnedChannelMessage" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePinnedChannelMessageSvc<T: GatewayRest>(pub Arc<T>);
                    impl < T : GatewayRest > tonic :: server :: UnaryService < super :: super :: super :: super :: discord :: v1 :: rest :: DeletePinnedChannelMessageRequest > for DeletePinnedChannelMessageSvc < T > { type Response = super :: super :: super :: super :: discord :: v1 :: rest :: DeletePinnedChannelMessageResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeletePinnedChannelMessageRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . delete_pinned_channel_message (request) . await } ; Box :: pin (fut) } }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeletePinnedChannelMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ListGuildEmojis" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildEmojisSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ListGuildEmojisRequest,
                        > for ListGuildEmojisSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::ListGuildEmojisResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ListGuildEmojisRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_guild_emojis(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuildEmojisSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetGuildEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildEmojiSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetGuildEmojiRequest,
                        > for GetGuildEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetGuildEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::GetGuildEmojiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_guild_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuildEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateGuildEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGuildEmojiSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateGuildEmojiRequest,
                        > for CreateGuildEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateGuildEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: CreateGuildEmojiRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_guild_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGuildEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyGuildEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyGuildEmojiSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyGuildEmojiRequest,
                        > for ModifyGuildEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::ModifyGuildEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyGuildEmojiRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_guild_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyGuildEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/DeleteGuildEmoji" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGuildEmojiSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::DeleteGuildEmojiRequest,
                        > for DeleteGuildEmojiSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::DeleteGuildEmojiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: DeleteGuildEmojiRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_guild_emoji(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteGuildEmojiSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetCurrentUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentUserSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetCurrentUserRequest,
                        > for GetCurrentUserSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetCurrentUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: GetCurrentUserRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_current_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCurrentUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::GetUserRequest,
                        > for GetUserSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::GetUserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::GetUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/ModifyCurrentUser" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyCurrentUserSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::ModifyCurrentUserRequest,
                        > for ModifyCurrentUserSvc<T>
                    {
                        type Response = super :: super :: super :: super :: discord :: v1 :: rest :: ModifyCurrentUserResponse ;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: discord :: v1 :: rest :: ModifyCurrentUserRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_current_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyCurrentUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/LeaveGuild" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveGuildSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::LeaveGuildRequest,
                        > for LeaveGuildSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::LeaveGuildResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::LeaveGuildRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).leave_guild(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LeaveGuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pylon.gateway.v1.service.GatewayRest/CreateDm" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDmSvc<T: GatewayRest>(pub Arc<T>);
                    impl<T: GatewayRest>
                        tonic::server::UnaryService<
                            super::super::super::super::discord::v1::rest::CreateDmRequest,
                        > for CreateDmSvc<T>
                    {
                        type Response =
                            super::super::super::super::discord::v1::rest::CreateDmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::discord::v1::rest::CreateDmRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_dm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GatewayRest> Clone for GatewayRestServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GatewayRest> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayRest> tonic::transport::NamedService for GatewayRestServer<T> {
        const NAME: &'static str = "pylon.gateway.v1.service.GatewayRest";
    }
}
