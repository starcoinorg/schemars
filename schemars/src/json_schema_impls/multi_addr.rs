use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use multiaddr::Multiaddr;
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