use chrono::{TimeZone, Utc};
use garde::Validate;
use serde_json::Value;
use werkstatt_call_shared::{
    CallDirection, CallEvent, CallNote, ClientCommand, ClientHello, ContactPerson, Customer,
    EnrichedCallEvent, LastContact, LinkSource, NoteCategory, OpenItem, OpenOrder, PhoneLink,
    ServerEvent, Vehicle,
};

// ---------------------------------------------------------------------------
// Helpers
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

// ---------------------------------------------------------------------------
// CallDirection
// ---------------------------------------------------------------------------

#[test]
fn call_direction_inbound_roundtrip() {
    let v = CallDirection::Inbound;
    let json = serde_json::to_string(&v).unwrap();
    let back: CallDirection = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    assert_eq!(json, r#""Inbound""#);
}

#[test]
fn call_direction_outbound_roundtrip() {
    let v = CallDirection::Outbound;
    let json = serde_json::to_string(&v).unwrap();
    let back: CallDirection = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    assert_eq!(json, r#""Outbound""#);
}

// ---------------------------------------------------------------------------
// CallEvent
// ---------------------------------------------------------------------------

#[test]
fn call_event_roundtrip() {
    let v = call_event();
    let json = serde_json::to_string(&v).unwrap();
    let back: CallEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn call_event_camel_case_keys() {
    let v = call_event();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("callId").is_some(), "expected callId key");
    assert!(obj.get("phoneNumber").is_some(), "expected phoneNumber key");
    assert!(obj.get("startedAt").is_some(), "expected startedAt key");
    assert!(obj.get("call_id").is_none(), "unexpected snake_case key");
}

// ---------------------------------------------------------------------------
// Customer
// ---------------------------------------------------------------------------

#[test]
fn customer_roundtrip() {
    let v = customer();
    let json = serde_json::to_string(&v).unwrap();
    let back: Customer = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn customer_camel_case_keys() {
    let v = customer();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(
        obj.get("customerNumber").is_some(),
        "expected customerNumber"
    );
    assert!(obj.get("city").is_some(), "expected city");
}

// ---------------------------------------------------------------------------
// Vehicle
// ---------------------------------------------------------------------------

#[test]
fn vehicle_roundtrip() {
    let v = vehicle();
    let json = serde_json::to_string(&v).unwrap();
    let back: Vehicle = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn vehicle_no_license_plate_roundtrip() {
    let v = Vehicle {
        vehicle_id: "veh-2".to_string(),
        label: "MAN TGX".to_string(),
        license_plate: None,
        next_hu_date: None,
        next_au_date: None,
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: Vehicle = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn vehicle_hu_au_camel_case_keys() {
    let v = vehicle();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("nextHuDate").is_some(), "expected nextHuDate");
    assert!(obj.get("nextAuDate").is_some(), "expected nextAuDate");
}

// ---------------------------------------------------------------------------
// OpenOrder
// ---------------------------------------------------------------------------

#[test]
fn open_order_roundtrip() {
    let v = open_order();
    let json = serde_json::to_string(&v).unwrap();
    let back: OpenOrder = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn open_order_camel_case_keys() {
    let v = open_order();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("orderNumber").is_some(), "expected orderNumber");
    assert!(obj.get("status").is_some(), "expected status");
}

// ---------------------------------------------------------------------------
// OpenItem
// ---------------------------------------------------------------------------

#[test]
fn open_item_roundtrip() {
    let v = open_item();
    let json = serde_json::to_string(&v).unwrap();
    let back: OpenItem = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

// ---------------------------------------------------------------------------
// LastContact
// ---------------------------------------------------------------------------

#[test]
fn last_contact_roundtrip() {
    let v = last_contact();
    let json = serde_json::to_string(&v).unwrap();
    let back: LastContact = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn last_contact_camel_case_keys() {
    let v = last_contact();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("contactType").is_some(), "expected contactType");
    assert!(obj.get("date").is_some(), "expected date");
}

// ---------------------------------------------------------------------------
// EnrichedCallEvent
// ---------------------------------------------------------------------------

#[test]
fn enriched_call_event_roundtrip() {
    let v = enriched_call_event();
    let json = serde_json::to_string(&v).unwrap();
    let back: EnrichedCallEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn enriched_call_event_unresolved_roundtrip() {
    let v = EnrichedCallEvent {
        call_event: call_event(),
        customer: None,
        vehicles: vec![],
        open_orders: vec![],
        last_contact: None,
        open_items: vec![],
        unresolved: false,
        werbas_url: None,
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: EnrichedCallEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn enriched_call_event_camel_case_keys() {
    let v = enriched_call_event();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("openOrders").is_some(), "expected openOrders");
    assert!(obj.get("lastContact").is_some(), "expected lastContact");
    assert!(obj.get("openItems").is_some(), "expected openItems");
    assert!(obj.get("werbasUrl").is_some(), "expected werbasUrl");
}

// ---------------------------------------------------------------------------
// NoteCategory
// ---------------------------------------------------------------------------

#[test]
fn note_category_all_variants_roundtrip() {
    let variants = [
        NoteCategory::Termin,
        NoteCategory::Panne,
        NoteCategory::Reklamation,
        NoteCategory::Angebot,
        NoteCategory::Sonstiges,
    ];
    for v in &variants {
        let json = serde_json::to_string(v).unwrap();
        let back: NoteCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(v, &back);
    }
}

// ---------------------------------------------------------------------------
// CallNote
// ---------------------------------------------------------------------------

#[test]
fn call_note_roundtrip() {
    let v = call_note();
    let json = serde_json::to_string(&v).unwrap();
    let back: CallNote = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn call_note_camel_case_keys() {
    let v = call_note();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("callId").is_some(), "expected callId");
    assert!(obj.get("createdAt").is_some(), "expected createdAt");
    assert!(obj.get("createdBy").is_some(), "expected createdBy");
    assert!(obj.get("orderRef").is_some(), "expected orderRef");
    assert!(obj.get("followUp").is_some(), "expected followUp");
}

#[test]
fn call_note_order_ref_key() {
    let v = call_note();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert_eq!(obj["orderRef"], "AUF-4711");
}

// ---------------------------------------------------------------------------
// LinkSource
// ---------------------------------------------------------------------------

#[test]
fn link_source_all_variants_roundtrip() {
    let variants = [
        LinkSource::Manual,
        LinkSource::AutoWerbas,
        LinkSource::Import,
    ];
    for v in &variants {
        let json = serde_json::to_string(v).unwrap();
        let back: LinkSource = serde_json::from_str(&json).unwrap();
        assert_eq!(v, &back);
    }
}

// ---------------------------------------------------------------------------
// ContactPerson
// ---------------------------------------------------------------------------

#[test]
fn contact_person_roundtrip() {
    let v = contact_person();
    let json = serde_json::to_string(&v).unwrap();
    let back: ContactPerson = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

// ---------------------------------------------------------------------------
// PhoneLink
// ---------------------------------------------------------------------------

#[test]
fn phone_link_roundtrip() {
    let v = phone_link();
    let json = serde_json::to_string(&v).unwrap();
    let back: PhoneLink = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn phone_link_auto_werbas_roundtrip() {
    let v = PhoneLink {
        phone_number: "+491711234567".to_string(),
        customer_id: "cust-42".to_string(),
        contact_person: None,
        role: None,
        source: LinkSource::AutoWerbas,
        confidence: 0.85,
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: PhoneLink = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn phone_link_customer_id_key() {
    let v = phone_link();
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(obj.get("customerId").is_some(), "expected customerId key");
    assert!(obj.get("customer_id").is_none(), "unexpected snake_case");
    assert!(obj.get("werbasCustomerId").is_none(), "unexpected old key");
}

// ---------------------------------------------------------------------------
// ClientHello
// ---------------------------------------------------------------------------

fn client_hello() -> ClientHello {
    ClientHello {
        hostname: "WORKSTATION-01".to_string(),
        user: "MS".to_string(),
        extensions: vec!["msgpack".to_string()],
        client_version: "0.1.0".to_string(),
        connected_at: fixed_dt(),
    }
}

#[test]
fn client_hello_roundtrip() {
    let v = client_hello();
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientHello = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn client_hello_camel_case_keys() {
    let obj: Value = serde_json::to_value(&client_hello()).unwrap();
    assert!(obj.get("clientVersion").is_some(), "expected clientVersion");
    assert!(obj.get("connectedAt").is_some(), "expected connectedAt");
    assert!(obj.get("user").is_some(), "expected user");
    assert!(
        obj.get("user_shorthand").is_none(),
        "unexpected user_shorthand"
    );
}

// ---------------------------------------------------------------------------
// ServerEvent
// ---------------------------------------------------------------------------

#[test]
fn server_event_incoming_call_roundtrip() {
    let v = ServerEvent::IncomingCall {
        event: enriched_call_event(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn server_event_call_ended_roundtrip() {
    let v = ServerEvent::CallEnded {
        call_id: "call-001".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn server_event_note_confirmed_roundtrip() {
    let v = ServerEvent::NoteConfirmed {
        call_id: "call-001".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn server_event_error_roundtrip() {
    let v = ServerEvent::Error {
        message: "internal error".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn server_event_tag_field_present() {
    let v = ServerEvent::CallEnded {
        call_id: "call-001".to_string(),
    };
    let obj: Value = serde_json::to_value(&v).unwrap();
    assert!(
        obj.get("type").is_some(),
        "tagged enum must include `type` field"
    );
    assert_eq!(obj["type"], "callEnded");
}

#[test]
fn server_event_phone_link_updated_roundtrip() {
    let v = ServerEvent::PhoneLinkUpdated { link: phone_link() };
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    let obj: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(obj["type"], "phoneLinkUpdated");
    assert!(obj.get("link").is_some(), "expected link field");
}

#[test]
fn server_event_heartbeat_roundtrip() {
    let v = ServerEvent::Heartbeat;
    let json = serde_json::to_string(&v).unwrap();
    let back: ServerEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    let obj: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(obj["type"], "heartbeat");
}

// ---------------------------------------------------------------------------
// ClientCommand
// ---------------------------------------------------------------------------

#[test]
fn client_command_save_note_roundtrip() {
    let v = ClientCommand::SaveNote {
        call_id: "call-001".to_string(),
        note: call_note(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn client_command_link_phone_roundtrip() {
    let v = ClientCommand::LinkPhone {
        phone_number: "+4989123456789".to_string(),
        customer_id: "cust-99".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn client_command_ack_roundtrip() {
    let v = ClientCommand::Ack {
        call_id: "call-001".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
}

#[test]
fn client_command_request_call_history_roundtrip() {
    let v = ClientCommand::RequestCallHistory {
        phone_number: "+4989123456789".to_string(),
    };
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    let obj: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(obj["type"], "requestCallHistory");
    assert_eq!(obj["phoneNumber"], "+4989123456789");
}

#[test]
fn client_command_ping_roundtrip() {
    let v = ClientCommand::Ping;
    let json = serde_json::to_string(&v).unwrap();
    let back: ClientCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(v, back);
    let obj: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(obj["type"], "ping");
}

// ---------------------------------------------------------------------------
// Validation — E.164 phone number
// ---------------------------------------------------------------------------

#[test]
fn valid_e164_phone_number_passes_validation() {
    let v = call_event();
    v.validate().expect("valid E.164 must pass");
}

#[test]
fn invalid_e164_phone_number_fails_validation() {
    let cases = [
        "04989123456789",     // missing leading +
        "+0123456789",        // leading 0 after +
        "+",                  // no digits
        "123456789",          // no + prefix
        "+12345678901234567", // too long (>15 digits total)
        "+1",                 // only country code, no subscriber number
    ];
    for bad in &cases {
        let v = CallEvent {
            phone_number: bad.to_string(),
            ..call_event()
        };
        assert!(
            v.validate().is_err(),
            "expected validation failure for: {bad}"
        );
    }
}

#[test]
fn valid_e164_minimum_subscriber_number_passes_validation() {
    // Minimum valid E.164 per spec: +[1-9] followed by at least 1 more digit
    let v = CallEvent {
        phone_number: "+12".to_string(), // country code 1 + single digit
        ..call_event()
    };
    v.validate()
        .expect("two-digit E.164 must pass (e.g. +12 as test value)");
}

#[test]
fn valid_e164_exactly_15_digits_passes_validation() {
    // Maximum valid E.164: 15 total digits
    let v = CallEvent {
        phone_number: "+123456789012345".to_string(), // 15 digits total
        ..call_event()
    };
    v.validate().expect("15-digit E.164 must pass");
}

// ---------------------------------------------------------------------------
// Validation — CallNote text length
// ---------------------------------------------------------------------------

#[test]
fn call_note_text_at_limit_passes_validation() {
    let v = CallNote {
        text: "x".repeat(5_000),
        ..call_note()
    };
    v.validate().expect("5000-byte text must pass");
}

#[test]
fn call_note_text_too_long_fails_validation() {
    let v = CallNote {
        text: "x".repeat(5_001),
        ..call_note()
    };
    assert!(v.validate().is_err(), "5001-byte text must fail");
}

// ---------------------------------------------------------------------------
// Validation — ContactPerson name length
// ---------------------------------------------------------------------------

#[test]
fn contact_person_name_at_limit_passes_validation() {
    let v = ContactPerson {
        name: "a".repeat(200),
        ..contact_person()
    };
    v.validate().expect("200-byte name must pass");
}

#[test]
fn contact_person_name_too_long_fails_validation() {
    let v = ContactPerson {
        name: "a".repeat(201),
        ..contact_person()
    };
    assert!(v.validate().is_err(), "201-byte name must fail");
}

// ---------------------------------------------------------------------------
// Validation — PhoneLink confidence range
// ---------------------------------------------------------------------------

#[test]
fn phone_link_confidence_bounds_pass_validation() {
    for conf in [0.0_f64, 0.5, 1.0] {
        let v = PhoneLink {
            confidence: conf,
            ..phone_link()
        };
        v.validate()
            .unwrap_or_else(|_| panic!("confidence={conf} must pass"));
    }
}

#[test]
fn phone_link_confidence_out_of_range_fails_validation() {
    for bad in [-0.1_f64, 1.1, 2.0] {
        let v = PhoneLink {
            confidence: bad,
            ..phone_link()
        };
        assert!(
            v.validate().is_err(),
            "confidence={bad} must fail validation"
        );
    }
}
