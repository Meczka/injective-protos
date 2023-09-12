// @generated
// ----------------------------------------
// Request types

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPing {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBroadcastTx {
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
// ----------------------------------------
// Response types

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePing {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBroadcastTx {
    #[prost(message, optional, tag="1")]
    pub check_tx: ::core::option::Option<super::super::abci::ResponseCheckTx>,
    #[prost(message, optional, tag="2")]
    pub deliver_tx: ::core::option::Option<super::super::abci::ResponseDeliverTx>,
}
include!("tendermint.rpc.grpc.tonic.rs");
// @@protoc_insertion_point(module)
