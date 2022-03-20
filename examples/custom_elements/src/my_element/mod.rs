use std::cell::Cell;
use std::convert::TryInto;

use arwa::console;
use arwa::dom::{name, selector, Element, ParentNode, ShadowHost, ShadowRootOptions};
use arwa::html::{
    default_adopted_callback, AttributeChange, CustomElement, CustomElementDescriptor,
    GenericExtendableElement, HtmlDocument, HtmlTemplateElement,
};
use arwa::window::window;

thread_local! {
    static TEMPLATE: HtmlTemplateElement = {
        let window = window().unwrap();
        let document: HtmlDocument = window.document().try_into().unwrap();

        let template_element: HtmlTemplateElement = document.create_known_element();

        template_element.deserialize_inner(include_str!("template.html"));

        template_element
    }
}

pub struct MyElementData {
    connected_count: Cell<u32>,
}

impl Drop for MyElementData {
    fn drop(&mut self) {
        console::log!("Dropping element data...")
    }
}

pub type MyElement = CustomElement<MyElementData, GenericExtendableElement>;

pub trait MyElementExt {
    fn message(&self) -> Option<String>;

    fn set_message(&self, message: &str);
}

impl MyElementExt for MyElement {
    fn message(&self) -> Option<String> {
        self.attributes()
            .lookup(&name!("message"))
            .map(|a| a.value())
    }

    fn set_message(&self, message: &str) {
        self.attributes().set(&name!("message"), message);
    }
}

fn constructor(extended: &GenericExtendableElement) -> MyElementData {
    let shadow_root = extended.attach_shadow(ShadowRootOptions::default());

    TEMPLATE.with(|template| {
        let content = ParentNode::duplicate_deep(&template.content());

        shadow_root.append_fragment(&content)
    });

    MyElementData {
        connected_count: Cell::new(0),
    }
}

fn connected_callback(element: &MyElement) {
    let connected_count = &element.data().connected_count;
    let count = connected_count.get() + 1;

    connected_count.set(count);

    console::log!("Custom element has been connected %i time(s)!", count);
}

fn disconnected_callback(_element: &MyElement) {
    console::log!("Disconnecting custom element...");
}

fn attribute_changed_callback(element: &MyElement, change: AttributeChange) {
    if change.attribute_name == "message" {
        let message_container = element
            .shadow_root()
            .unwrap()
            .query_selector_first(&selector!("#message_container"))
            .unwrap();

        message_container.deserialize_inner(&change.new_value.unwrap_or_default());
    }
}

pub const MY_ELEMENT: CustomElementDescriptor<MyElementData, GenericExtendableElement> =
    CustomElementDescriptor {
        constructor,
        connected_callback,
        disconnected_callback,
        adopted_callback: default_adopted_callback,
        attribute_changed_callback,
        observed_attributes: &[name!("message")],
    };
