use bevy_fluent::prelude::*;
use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

#[test]
fn use_id() {
    let bundle = build_bundle();
    let content = bundle.content("one");
    assert!(matches!(content, Some(content) if content == "One"));
}

#[test]
fn use_id_attr() {
    let bundle = build_bundle();
    let content = bundle.content("one.two");
    assert!(matches!(content, Some(content) if content == "Two"));
}

#[test]
fn use_id_args() {
    let bundle = build_bundle();
    let content = bundle.content("three?arg1=first");
    println!("content: {:?}", content);
    assert!(matches!(content, Some(content) if content == "Three \u{2068}first\u{2069}"));
}

#[test]
fn use_id_attr_args() {
    let bundle = build_bundle();
    let content = bundle.content("three.four?arg2=second");
    println!("content: {:?}", content.as_ref().unwrap());
    assert!(matches!(content, Some(content) if content == "Four \u{2068}second\u{2069}"));
}

fn build_bundle() -> FluentBundle<FluentResource, IntlLangMemoizer> {
    let mut bundle = FluentBundle::new_concurrent(vec![]);
    let resource = FluentResource::try_new(String::from(
        "one = One\n\
            .two = Two\n\
        three = Three { $arg1 }\n\
            .four = Four { $arg2 }\n\
        ",
    ))
    .unwrap();
    bundle.add_resource(resource).unwrap();
    bundle
}
