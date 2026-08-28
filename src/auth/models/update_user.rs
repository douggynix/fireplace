use serde::Serialize;
use serde_with::skip_serializing_none;
use typed_builder_macro::TypedBuilder;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserValues {
    #[builder(default = None, setter(into))]
    display_name: Option<String>,
    #[builder(default = None, setter(into))]
    email: Option<String>,
    #[builder(default = None, setter(into))]
    password: Option<String>,
    #[builder(default = None, setter(into))]
    #[serde(rename = "disableUser")]
    disabled: Option<bool>,
    #[builder(default = None, setter(into))]
    email_verified: Option<bool>,
    #[builder(default = None, setter(into))]
    pub phone_number: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateUserBody<'a> {
    local_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = None, setter(into))]
    email_verified: Option<bool>,
    #[builder(default = None, setter(into))]
    pub phone_number: Option<String>,
}

impl<'a> UpdateUserBody<'a> {
    pub(crate) fn from_values(user_id: &'a str, values: UpdateUserValues) -> Self {
        UpdateUserBody::builder()
            .local_id(user_id)
            .display_name(values.display_name)
            .email(values.email)
            .password(values.password)
            .disable_user(values.disabled)
            .email_verified(values.email_verified)
            .phone_number(values.phone_number)
            .build()
    }
}
