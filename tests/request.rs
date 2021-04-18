use bevy_fluent::exts::bundle::Request;
use fluent::types::FluentValue;
use std::mem::discriminant;

#[test]
fn parse_id() {
    assert!(matches!(Request::from("id"), Request { id: "id", .. }));
}

#[test]
fn parse_id_attr() {
    assert!(matches!(
        Request::from("id.attr"),
        Request {
            id: "id",
            attr: Some("attr"),
            ..
        }
    ));
}

#[test]
fn parse_id_args() {
    assert!(matches!(
        Request::from("id?key=value"),
        Request {
            id: "id",
            args: Some(args),
            ..
        } if args.get("key") == Some(&"value".into())
    ));
    assert!(matches!(
        Request::from("id?key1=value1&key2=value2"),
        Request {
            id: "id",
            args: Some(args),
            ..
        } if args.get("key1") == Some(&"value1".into())
            && args.get("key2") == Some(&"value2".into())
            && args.get("key3") == None
    ));
}

#[test]
fn parse_id_attr_args() {
    assert!(matches!(
        Request::from("id.attr?key=value"),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if args.get("key") == Some(&"value".into())
    ));
    assert!(matches!(
        Request::from("id.attr?key1=value1&key2=value2"),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if args.get("key1") == Some(&"value1".into())
            && args.get("key2") == Some(&"value2".into())
            && args.get("key3") == None
    ));
}

#[test]
fn parse_none_args() {
    assert!(matches!(
        Request::from("id.attr?key="),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if matches!(
            args.get("key"),
            Some(key) if discriminant(key) == discriminant(&FluentValue::None)
        )
    ));
}

#[test]
fn parse_error_args() {
    assert!(matches!(
        Request::from("id.attr?key"),
        Request {
            id: "id",
            attr: Some("attr"),
            args: Some(args),
        } if matches!(
            args.get("key"),
            Some(key) if discriminant(key) == discriminant(&FluentValue::Error)
        )
    ));
}
