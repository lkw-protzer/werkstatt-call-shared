//! MessagePack round-trip tests.
//!
//! All tests in this file require the `msgpack` Cargo feature.
//! Run with: `cargo test --features msgpack`
#![cfg(feature = "msgpack")]

use chrono::{TimeZone, Utc};
use werkstatt_call_shared::{
    decode, encode, CallDirection, CallEvent, CallNote, ClientCommand, ClientHello, ContactPerson,
    Customer, EnrichedCallEvent, LastContact, LinkSource, NoteCategory, OpenItem, OpenOrder,
    PhoneLink, ServerEvent, Vehicle, WireFormat,
};

// ---------------------------------------------------------------------------
// Helpers (mirrors roundtrip.rs fixtures)
// ---------------------------------------------------------------------------

fn fixed_dt() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap()
}

fn call_event() -> CallEvent {
    CallEvent {
        call_id: "call-001".to_string(),
        direction: CallDirection::Inbound,
        phone_number: "+4989123456789".to_string(),
        started_at: fixed_dt(),
        extension: "42".to_string(),
    }
}

fn customer() -> Customer {
    Customer {
        id: "cust-99".to_string(),
        name: "Max Mustermann GmbH".to_string(),
        customer_number: Some("KD-12345".to_string()),
        city: Some("München".to_string()),
    }
}

fn vehicle() -> Vehicle {
    Vehicle {
        vehicle_id: "veh-1".to_string(),
        label: "Mercedes Actros 2021".to_string(),
        license_plate: Some("M-XY 1234".to_string()),
        next_hu_date: Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()),
        next_au_date: None,
    }
}

fn open_order() -> OpenOrder {
    OpenOrder {
        id: "ord-001".to_string(),
        order_number: "AUF-4711".to_string(),
        description: Some("Ölwechsel".to_string()),
        status: "Offen".to_string(),
    }
}

fn open_item() -> OpenItem {
    OpenItem {
        id: "item-001".to_string(),
        amount_eur: 250.0,
        label: Some("RE-2024-001".to_string()),
    }
}

fn last_contact() -> LastContact {
    LastContact {
        date: fixed_dt(),
        contact_type: "Anruf".to_string(),
    }
}

fn enriched_call_event() -> EnrichedCallEvent {
    EnrichedCallEvent {
        call_event: call_event(),
        customer: Some(customer()),
        vehicles: vec![vehicle()],
        open_orders: vec![open_order()],
        last_contact: Some(last_contact()),
        open_items: vec![open_item()],
        unresolved: true,
        suggestions: vec![],
        werbas_url: Some("http://werbas.local/customer/cust-99".to_string()),
    }
}

fn contact_person() -> ContactPerson {
    ContactPerson {
        name: "Erika Muster".to_string(),
        role: Some("Disposition".to_string()),
        notes: None,
    }
}

fn phone_link() -> PhoneLink {
    PhoneLink {
        phone_number: "+4989123456789".to_string(),
        customer_id: "cust-99".to_string(),
        contact_person: Some(contact_person()),
        role: None,
        source: LinkSource::Manual,
        confidence: 1.0,
    }
}

fn call_note() -> CallNote {
    CallNote {
        call_id: "call-001".to_string(),
        created_at: fixed_dt(),
        created_by: "MS".to_string(),
        category: NoteCategory::Termin,
        text: "Termin am Donnerstag vereinbart.".to_string(),
        follow_up: false,
        order_ref: Some("AUF-4711".to_string()),
    }
}

fn client_hello() -> ClientHello {
    ClientHello {
        hostname: "WORKSTATION-01".to_string(),
        user: "MS".to_string(),
        extensions: vec!["msgpack".to_string()],
        client_version: "0.1.0".to_string(),
        connected_at: fixed_dt(),
        preferred_format: WireFormat::MsgPack,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn msgpack_roundtrip<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let bytes = encode(value, WireFormat::MsgPack).expect("encode must succeed");
    assert!(!bytes.is_empty(), "encoded bytes must not be empty");
    decode(&bytes, WireFormat::MsgPack).expect("decode must succeed")
}

// ---------------------------------------------------------------------------
// WireFormat itself
// ---------------------------------------------------------------------------

#[test]
fn wire_format_json_msgpack_roundtrip() {
    let v = WireFormat::MsgPack;
    let back = msgpack_roundtrip(&v);
    assert_eq!(v, back);
}

#[test]
fn wire_format_json_default_roundtrip() {
    let v = WireFormat::Json;
    let back = msgpack_roundtrip(&v);
    assert_eq!(v, back);
}

// ---------------------------------------------------------------------------
// CallEvent
// ---------------------------------------------------------------------------

#[test]
fn call_event_msgpack_roundtrip() {
    let v = call_event();
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// EnrichedCallEvent
// ---------------------------------------------------------------------------

#[test]
fn enriched_call_event_msgpack_roundtrip() {
    let v = enriched_call_event();
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn enriched_call_event_empty_msgpack_roundtrip() {
    let v = EnrichedCallEvent {
        call_event: call_event(),
        customer: None,
        vehicles: vec![],
        open_orders: vec![],
        last_contact: None,
        open_items: vec![],
        unresolved: false,
        suggestions: vec![],
        werbas_url: None,
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// CallNote
// ---------------------------------------------------------------------------

#[test]
fn call_note_msgpack_roundtrip() {
    let v = call_note();
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// PhoneLink
// ---------------------------------------------------------------------------

#[test]
fn phone_link_msgpack_roundtrip() {
    let v = phone_link();
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// ClientHello
// ---------------------------------------------------------------------------

#[test]
fn client_hello_msgpack_roundtrip() {
    let v = client_hello();
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn client_hello_default_format_msgpack_roundtrip() {
    let v = ClientHello {
        preferred_format: WireFormat::Json,
        ..client_hello()
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// ServerEvent (internally tagged enum — uses to_vec_named)
// ---------------------------------------------------------------------------

#[test]
fn server_event_incoming_call_msgpack_roundtrip() {
    let v = ServerEvent::IncomingCall {
        event: enriched_call_event(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn server_event_call_ended_msgpack_roundtrip() {
    let v = ServerEvent::CallEnded {
        call_id: "call-001".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn server_event_note_confirmed_msgpack_roundtrip() {
    let v = ServerEvent::NoteConfirmed {
        call_id: "call-001".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn server_event_error_msgpack_roundtrip() {
    let v = ServerEvent::Error {
        message: "internal error".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn server_event_phone_link_updated_msgpack_roundtrip() {
    let v = ServerEvent::PhoneLinkUpdated { link: phone_link() };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn server_event_heartbeat_msgpack_roundtrip() {
    let v = ServerEvent::Heartbeat;
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// ClientCommand (internally tagged enum — uses to_vec_named)
// ---------------------------------------------------------------------------

#[test]
fn client_command_save_note_msgpack_roundtrip() {
    let v = ClientCommand::SaveNote {
        call_id: "call-001".to_string(),
        note: call_note(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn client_command_link_phone_msgpack_roundtrip() {
    let v = ClientCommand::LinkPhone {
        phone_number: "+4989123456789".to_string(),
        customer_id: "cust-99".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn client_command_ack_msgpack_roundtrip() {
    let v = ClientCommand::Ack {
        call_id: "call-001".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn client_command_request_call_history_msgpack_roundtrip() {
    let v = ClientCommand::RequestCallHistory {
        phone_number: "+4989123456789".to_string(),
    };
    assert_eq!(v, msgpack_roundtrip(&v));
}

#[test]
fn client_command_ping_msgpack_roundtrip() {
    let v = ClientCommand::Ping;
    assert_eq!(v, msgpack_roundtrip(&v));
}

// ---------------------------------------------------------------------------
// encode/decode returns UnsupportedFormat error when feature is missing
// (not applicable here since this file only compiles with feature = "msgpack")
// Instead verify that JSON still works via encode/decode helpers
// ---------------------------------------------------------------------------

#[test]
fn encode_decode_json_still_works() {
    let v = call_event();
    let bytes = encode(&v, WireFormat::Json).expect("JSON encode must succeed");
    let back: CallEvent = decode(&bytes, WireFormat::Json).expect("JSON decode must succeed");
    assert_eq!(v, back);
}
