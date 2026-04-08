//! OpenAPI schema generation binary.
//!
//! Generates an OpenAPI 3.1 document from the `utoipa` `ToSchema` derives on
//! all public types and writes it to `schema/openapi.yaml`.
//!
//! Run with:
//!   cargo run --bin generate-openapi
//!
//! The output should be compared against (and committed alongside) the
//! handwritten `schema/openapi.yaml` to detect schema drift.

use std::io;

use utoipa::OpenApi;
use werkstatt_call_shared::{
    CallDirection, CallEvent, CallNote, ClientCommand, ClientHello, ContactPerson, Customer,
    EnrichedCallEvent, LastContact, LinkSource, NoteCategory, OpenItem, OpenOrder, PhoneLink,
    ServerEvent, Vehicle,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "werkstatt-call Shared Schemas",
        version = "0.1.0",
        description = "Component schemas shared between werkstatt-call-server (REST + WebSocket) \
                       and werkstatt-call-client. These schemas are the authoritative definitions \
                       derived from the Rust types in werkstatt-call-shared."
    ),
    components(schemas(
        CallDirection,
        CallEvent,
        Customer,
        Vehicle,
        OpenOrder,
        OpenItem,
        LastContact,
        EnrichedCallEvent,
        NoteCategory,
        CallNote,
        LinkSource,
        ContactPerson,
        PhoneLink,
        ClientHello,
        ServerEvent,
        ClientCommand,
    ))
)]
struct ApiDoc;

fn main() -> io::Result<()> {
    let yaml = ApiDoc::openapi()
        .to_yaml()
        .expect("OpenAPI YAML serialization failed");

    std::fs::create_dir_all("schema")?;
    std::fs::write("schema/openapi.generated.yaml", &yaml)?;
    eprintln!("Written: schema/openapi.generated.yaml");
    eprintln!("Compare against schema/openapi.yaml to detect drift.");
    Ok(())
}
