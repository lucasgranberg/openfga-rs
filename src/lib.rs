pub use prost_types;
pub use prost_wkt_types;
pub use tonic;

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTypeFilter {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub relation: Option<::prost::alloc::string::String>,
}

include!(concat!(env!("OUT_DIR"), "/openfga.v1.rs"));
