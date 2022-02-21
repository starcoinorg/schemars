use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use starcoin_crypto::HashValue;
use move_core_types::account_address::AccountAddress;
use multiaddr::Multiaddr;
impl JsonSchema for AccountAddress {
    no_ref_schema!();

    fn schema_name() -> String {
        "AccountAddress".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("AccountAddress".to_owned()),
            ..Default::default()
        }
        .into()
    }
}
impl JsonSchema for HashValue {
    no_ref_schema!();

    fn schema_name() -> String {
        "HashValue".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("HashValue".to_owned()),
            ..Default::default()
        }
        .into()
    }
}
impl JsonSchema for Multiaddr {
    no_ref_schema!();

    fn schema_name() -> String {
        "Multiaddr".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("Multiaddr".to_owned()),
            ..Default::default()
        }
        .into()
    }
}
