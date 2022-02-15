use crate::html::{AutoComplete, form_listed_element_seal, FormListedElement, HtmlFormElement, DynamicFormListedElement, constraint_validation_target_seal, ConstraintValidationTarget, ValidityState, HtmlOptionElement};
use crate::InvalidCast;
use std::convert::TryFrom;
use crate::collection::{Collection, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectType {
    SelectOne,
    SelectMultiple,
}

#[derive(Clone)]
pub struct HtmlSelectElement {
    inner: web_sys::HtmlSelectElement,
}

impl HtmlSelectElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn autofocus(&self) -> bool;

            pub fn set_autofocus(&self, autofocus: bool);

            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn multiple(&self) -> bool;

            pub fn set_multiple(&self, multiple: bool);

            pub fn required(&self) -> bool;

            pub fn set_required(&self, required: bool);

            pub fn size(&self) -> u32;

            pub fn set_size(&self, size: u32);
        }
    }

    pub fn autocomplete(&self) -> AutoComplete {
        match &*self.inner.autocomplete() {
            "off" => AutoComplete::Off,
            _ => AutoComplete::On,
        }
    }

    pub fn set_autocomplete(&self, autocomplete: AutoComplete) {
        let autocomplete = match autocomplete {
            AutoComplete::On => "on",
            AutoComplete::Off => "off",
        };

        self.inner.set_autocomplete(autocomplete);
    }

    pub fn select_type(&self) -> SelectType {
        match &*self.inner.type_() {
            "select-one" => SelectType::SelectOne,
            "select-multiple" => SelectType::SelectMultiple,
            _ => unreachable!(),
        }
    }

    pub fn options(&self) -> SelectOptions {
        SelectOptions {
            inner: self.inner.options(),
        }
    }

    pub fn selected_options(&self) -> SelectSelectedOptions {
        SelectSelectedOptions {
            inner: self.inner.selected_options(),
        }
    }

    // Note: ignoring `selectedIndex`, prefer `selected_options().first()` instead.

    // Note: ignoring the ability to modify the options through `SelectOptions`, prefer modifying
    // through the general `ParentNode`/`ChildNode` interfaces.
}

impl form_listed_element_seal::Seal for HtmlSelectElement {}

impl FormListedElement for HtmlSelectElement {
    delegate! {
        to self.inner {
            fn name(&self) -> String;

            fn set_name(&self, name: &str);
        }
    }

    fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }
}

impl TryFrom<DynamicFormListedElement> for HtmlSelectElement {
    type Error = InvalidCast<DynamicFormListedElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlObjectElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl constraint_validation_target_seal::Seal for HtmlSelectElement {}

impl ConstraintValidationTarget for HtmlSelectElement {
    delegate! {
        to self.inner {
            fn will_validate(&self) -> bool;

            fn check_validity(&self) -> bool;

            fn report_validity(&self) -> bool;

            fn set_custom_validity(&self, error: &str);
        }
    }

    fn validity(&self) -> ValidityState {
        self.inner.validity().into()
    }

    fn validation_message(&self) -> String {
        self.inner.validation_message().unwrap_or(String::new())
    }
}

impl From<web_sys::HtmlSelectElement> for HtmlSelectElement {
    fn from(inner: web_sys::HtmlSelectElement) -> Self {
        HtmlSelectElement { inner }
    }
}

impl AsRef<web_sys::HtmlSelectElement> for HtmlSelectElement {
    fn as_ref(&self) -> &web_sys::HtmlSelectElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlSelectElement);
impl_try_from_element!(HtmlSelectElement);
impl_known_element!(HtmlSelectElement, "SELECT");

#[derive(Clone, Copy)]
pub struct SelectOptions {
    inner: web_sys::HtmlOptionsCollection,
}

impl Collection for SelectOptions {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for SelectOptions {
    type Item = HtmlOptionElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|o| HtmlOptionElement::from(o.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub struct SelectSelectedOptions {
    inner: web_sys::HtmlCollection,
}

impl Collection for SelectSelectedOptions {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for SelectSelectedOptions {
    type Item = HtmlOptionElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|o| HtmlOptionElement::from(o.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
