use crate::database::models::loader_fields::{
    LoaderFieldEnumValue, VersionField, VersionFieldValue,
};
use crate::models::projects::Loader;
use crate::validate::{ValidationError, ValidationResult};
use bytes::Bytes;
use std::borrow::Cow;

const SUPPORTED_LOADERS: &[&str] = &["shroudtopia", "shroudforge", "eml"];

fn invalid(message: impl Into<String>) -> ValidationError {
    ValidationError::InvalidInput(Cow::Owned(message.into()))
}

fn integer_field(fields: &[VersionField], name: &str) -> Option<i32> {
    fields.iter().find_map(|field| {
        (field.field_name == name)
            .then_some(&field.value)
            .and_then(|value| match value {
                VersionFieldValue::Integer(value) => Some(*value),
                _ => None,
            })
    })
}

fn text_field<'a>(fields: &'a [VersionField], name: &str) -> Option<&'a str> {
    fields.iter().find_map(|field| {
        (field.field_name == name)
            .then_some(&field.value)
            .and_then(|value| match value {
                VersionFieldValue::Text(value) => Some(value.as_str()),
                _ => None,
            })
    })
}

fn game_versions(fields: &[VersionField]) -> Option<&[LoaderFieldEnumValue]> {
    fields.iter().find_map(|field| {
        (field.field_name == "game_versions")
            .then_some(&field.value)
            .and_then(|value| match value {
                VersionFieldValue::ArrayEnum(_, values) => {
                    Some(values.as_slice())
                }
                _ => None,
            })
    })
}

pub async fn validate_file(
    data: Bytes,
    loaders: Vec<Loader>,
    fields: Vec<VersionField>,
) -> Result<ValidationResult, ValidationError> {
    if data.is_empty() {
        return Err(invalid("schematic file cannot be empty"));
    }

    if loaders.is_empty()
        || loaders
            .iter()
            .any(|loader| !SUPPORTED_LOADERS.contains(&loader.0.as_str()))
    {
        return Err(invalid(
            "schematic loaders must be `shroudtopia`, `shroudforge`, or `eml`",
        ));
    }

    if game_versions(&fields).is_none_or(|versions| versions.len() != 1) {
        return Err(invalid("schematics must target exactly one game version"));
    }

    if integer_field(&fields, "schematic_format_version")
        .is_none_or(|value| value < 1)
    {
        return Err(invalid("schematic format version must be at least 1"));
    }

    if text_field(&fields, "world_editor_version")
        .is_none_or(|value| value.trim().is_empty())
    {
        return Err(invalid("world editor version cannot be empty"));
    }

    for field in ["schematic_width", "schematic_height", "schematic_depth"] {
        if integer_field(&fields, field)
            .is_none_or(|value| !(1..=4096).contains(&value))
        {
            return Err(invalid(
                "schematic dimensions must be between 1 and 4096",
            ));
        }
    }

    Ok(ValidationResult::Pass)
}
