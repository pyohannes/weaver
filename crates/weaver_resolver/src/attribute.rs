// SPDX-License-Identifier: Apache-2.0

//! Attribute resolution.

use std::collections::{BTreeMap, HashMap, HashSet};

use serde::Deserialize;

use weaver_resolved_schema::attribute;
use weaver_resolved_schema::attribute::AttributeRef;
use weaver_resolved_schema::lineage::{AttributeLineage, GroupLineage};
use weaver_schema::attribute::Attribute;
use weaver_schema::tags::Tags;
use weaver_semconv::attribute::{AttributeSpec, ValueSpec};
use weaver_semconv::group::GroupType;
use weaver_semconv::SemConvRegistry;
use weaver_version::VersionAttributeChanges;

use crate::Error;

/// A catalog of deduplicated resolved attributes with their corresponding reference.
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct AttributeCatalog {
    /// A map of deduplicated resolved attributes with their corresponding reference.
    attribute_refs: HashMap<attribute::Attribute, AttributeRef>,
    #[serde(skip)]
    /// A map of root attributes indexed by their name.
    /// Root attributes are attributes that doesn't inherit from another attribute.
    root_attributes: HashMap<String, AttributeWithGroupId>,
}

#[derive(Debug, PartialEq)]
struct AttributeWithGroupId {
    pub attribute: attribute::Attribute,
    pub group_id: String,
}

impl AttributeCatalog {
    /// Returns the reference of the given attribute or creates a new reference if the attribute
    /// does not exist in the catalog.
    pub fn attribute_ref(&mut self, attr: attribute::Attribute) -> AttributeRef {
        let next_id = self.attribute_refs.len() as u32;
        *self
            .attribute_refs
            .entry(attr)
            .or_insert_with(|| AttributeRef(next_id))
    }

    /// Returns a list of deduplicated attributes ordered by their references.
    #[must_use]
    pub fn drain_attributes(self) -> Vec<attribute::Attribute> {
        let mut attributes: Vec<(attribute::Attribute, AttributeRef)> =
            self.attribute_refs.into_iter().collect();
        attributes.sort_by_key(|(_, attr_ref)| attr_ref.0);
        attributes.into_iter().map(|(attr, _)| attr).collect()
    }

    /// Returns a list of indexed attribute names ordered by their references.
    #[must_use]
    pub fn attribute_name_index(&self) -> Vec<String> {
        let mut attributes: Vec<(&attribute::Attribute, &AttributeRef)> =
            self.attribute_refs.iter().collect();
        attributes.sort_by_key(|(_, attr_ref)| attr_ref.0);
        attributes
            .iter()
            .map(|(attr, _)| attr.name.clone())
            .collect()
    }

    /// Tries to resolve the given attribute spec (ref or id) from the catalog.
    /// Returns `None` if the attribute spec is a ref and it does not exist yet
    /// in the catalog.
    pub fn resolve(
        &mut self,
        group_id: &str,
        prefix: &str,
        attr: &AttributeSpec,
        lineage: Option<&mut GroupLineage>,
    ) -> Option<AttributeRef> {
        match attr {
            AttributeSpec::Ref {
                r#ref,
                brief,
                examples,
                tag,
                requirement_level,
                sampling_relevant,
                note,
                stability,
                deprecated,
            } => {
                let root_attr = self.root_attributes.get(r#ref);
                if let Some(root_attr) = root_attr {
                    let mut attr_lineage = AttributeLineage::new(&root_attr.group_id);

                    // Create a fully resolved attribute from an attribute spec
                    // (ref) and override the root attribute with the new
                    // values if they are present.
                    let resolved_attr = attribute::Attribute {
                        name: r#ref.clone(),
                        r#type: root_attr.attribute.r#type.clone(),
                        brief: attr_lineage.brief(brief, &root_attr.attribute.brief),
                        examples: attr_lineage.examples(examples, &root_attr.attribute.examples),
                        tag: attr_lineage.tag(tag, &root_attr.attribute.tag),
                        requirement_level: attr_lineage.requirement_level(
                            requirement_level,
                            &root_attr.attribute.requirement_level,
                        ),
                        sampling_relevant: attr_lineage.sampling_relevant(
                            sampling_relevant,
                            &root_attr.attribute.sampling_relevant,
                        ),
                        note: attr_lineage.note(note, &root_attr.attribute.note),
                        stability: attr_lineage
                            .stability(stability, &root_attr.attribute.stability),
                        deprecated: attr_lineage
                            .deprecated(deprecated, &root_attr.attribute.deprecated),
                        tags: root_attr.attribute.tags.clone(),
                        value: root_attr.attribute.value.clone(),
                    };

                    let attr_ref = self.attribute_ref(resolved_attr);

                    // Update the lineage based on the inherited fields.
                    // Note: the lineage is only updated if a group lineage is provided.
                    if let Some(lineage) = lineage {
                        lineage.add_attribute_lineage(r#ref.to_owned(), attr_lineage);
                    }

                    Some(attr_ref)
                } else {
                    None
                }
            }
            AttributeSpec::Id {
                id,
                r#type,
                brief,
                examples,
                tag,
                requirement_level,
                sampling_relevant,
                note,
                stability,
                deprecated,
            } => {
                let root_attr_id = if prefix.is_empty() {
                    id.clone()
                } else {
                    format!("{}.{}", prefix, id)
                };

                // Create a fully resolved attribute from an attribute spec (id),
                // and check if it already exists in the catalog.
                // If it does, return the reference to the existing attribute.
                // If it does not, add it to the catalog and return a new reference.
                let attr = attribute::Attribute {
                    name: root_attr_id.clone(),
                    r#type: r#type.clone(),
                    brief: brief.clone().unwrap_or_default(),
                    examples: examples.clone(),
                    tag: tag.clone(),
                    requirement_level: requirement_level.clone(),
                    sampling_relevant: *sampling_relevant,
                    note: note.clone(),
                    stability: stability.clone(),
                    deprecated: deprecated.clone(),
                    tags: None,
                    value: None,
                };

                _ = self.root_attributes.insert(
                    root_attr_id,
                    AttributeWithGroupId {
                        attribute: attr.clone(),
                        group_id: group_id.to_owned(),
                    },
                );
                Some(self.attribute_ref(attr))
            }
        }
    }
}

/// Resolves a collection of attributes (i.e. `Attribute::Ref`, `Attribute::AttributeGroupRef`,
/// and `Attribute::SpanRef`) from the given semantic convention catalog and local attributes
/// (i.e. `Attribute::Id`).
/// `Attribute::AttributeGroupRef` are first resolved, then `Attribute::SpanRef`, then
/// `Attribute::Ref`, and finally `Attribute::Id` are added.
/// An `Attribute::Ref` can override an attribute contained in an `Attribute::AttributeGroupRef`
/// or an `Attribute::SpanRef`.
/// An `Attribute::Id` can override an attribute contains in an `Attribute::Ref`, an
/// `Attribute::AttributeGroupRef`, or an `Attribute::SpanRef`.
///
/// Note: Version changes are used during the resolution process to determine the names of the
/// attributes.
pub fn resolve_attributes(
    attributes: &[Attribute],
    semconv_registry: &SemConvRegistry,
    version_changes: impl VersionAttributeChanges,
) -> Result<Vec<Attribute>, Error> {
    let mut resolved_attrs = BTreeMap::new();
    let mut copy_into_resolved_attrs =
        |attrs: HashMap<&String, &AttributeSpec>, tags: &Option<Tags>| {
            for (attr_id, attr) in attrs {
                let mut attr: Attribute = attr.into();
                attr.set_tags(tags);
                _ = resolved_attrs.insert(attr_id.clone(), attr);
            }
        };

    // Resolve `Attribute::AttributeGroupRef`
    for attribute in attributes.iter() {
        if let Attribute::AttributeGroupRef {
            attribute_group_ref,
            tags,
        } = attribute
        {
            let attrs = semconv_registry
                .attributes(attribute_group_ref, GroupType::AttributeGroup)
                .map_err(|e| Error::FailToResolveAttributes {
                    ids: vec![attribute_group_ref.clone()],
                    error: e.to_string(),
                })?;
            copy_into_resolved_attrs(attrs, tags);
        }
    }

    // Resolve `Attribute::ResourceRef`
    for attribute in attributes.iter() {
        if let Attribute::ResourceRef { resource_ref, tags } = attribute {
            let attrs = semconv_registry
                .attributes(resource_ref, GroupType::Resource)
                .map_err(|e| Error::FailToResolveAttributes {
                    ids: vec![resource_ref.clone()],
                    error: e.to_string(),
                })?;
            copy_into_resolved_attrs(attrs, tags);
        }
    }

    // Resolve `Attribute::SpanRef`
    for attribute in attributes.iter() {
        if let Attribute::SpanRef { span_ref, tags } = attribute {
            let attrs = semconv_registry
                .attributes(span_ref, GroupType::Span)
                .map_err(|e| Error::FailToResolveAttributes {
                    ids: vec![span_ref.clone()],
                    error: e.to_string(),
                })?;
            copy_into_resolved_attrs(attrs, tags);
        }
    }

    // Resolve `Attribute::EventRef`
    for attribute in attributes.iter() {
        if let Attribute::EventRef { event_ref, tags } = attribute {
            let attrs = semconv_registry
                .attributes(event_ref, GroupType::Event)
                .map_err(|e| Error::FailToResolveAttributes {
                    ids: vec![event_ref.clone()],
                    error: e.to_string(),
                })?;
            copy_into_resolved_attrs(attrs, tags);
        }
    }

    // Resolve `Attribute::Ref`
    for attribute in attributes.iter() {
        if let Attribute::Ref { r#ref, .. } = attribute {
            let normalized_ref = version_changes.get_attribute_name(r#ref);
            let sem_conv_attr = semconv_registry.attribute(&normalized_ref);
            let resolved_attribute = attribute.resolve_from(sem_conv_attr).map_err(|e| {
                Error::FailToResolveAttributes {
                    ids: vec![r#ref.clone()],
                    error: e.to_string(),
                }
            })?;
            _ = resolved_attrs.insert(normalized_ref, resolved_attribute);
        }
    }

    // Resolve `Attribute::Id`
    // Note: any resolved attributes with the same id will be overridden.
    for attribute in attributes.iter() {
        if let Attribute::Id { id, .. } = attribute {
            _ = resolved_attrs.insert(id.clone(), attribute.clone());
        }
    }

    Ok(resolved_attrs.into_values().collect())
}

/// Merges the given main attributes with the inherited attributes.
/// Main attributes have precedence over inherited attributes.
#[must_use]
pub fn merge_attributes(main_attrs: &[Attribute], inherited_attrs: &[Attribute]) -> Vec<Attribute> {
    let mut merged_attrs = main_attrs.to_vec();
    let main_attr_ids = main_attrs
        .iter()
        .map(|attr| match attr {
            Attribute::Ref { r#ref, .. } => r#ref.clone(),
            Attribute::Id { id, .. } => id.clone(),
            Attribute::AttributeGroupRef { .. } => {
                panic!("Attribute groups are not supported yet")
            }
            Attribute::SpanRef { .. } => {
                panic!("Span references are not supported yet")
            }
            Attribute::ResourceRef { .. } => {
                panic!("Resource references are not supported yet")
            }
            Attribute::EventRef { .. } => {
                panic!("Event references are not supported yet")
            }
        })
        .collect::<HashSet<_>>();

    for inherited_attr in inherited_attrs.iter() {
        match inherited_attr {
            Attribute::Ref { r#ref, .. } => {
                if main_attr_ids.contains(r#ref) {
                    continue;
                }
            }
            Attribute::Id { id, .. } => {
                if main_attr_ids.contains(id) {
                    continue;
                }
            }
            Attribute::AttributeGroupRef { .. } => {
                panic!("Attribute groups are not supported yet")
            }
            Attribute::SpanRef { .. } => {
                panic!("Span references are not supported yet")
            }
            Attribute::ResourceRef { .. } => {
                panic!("Resource references are not supported yet")
            }
            Attribute::EventRef { .. } => {
                panic!("Event references are not supported yet")
            }
        }
        merged_attrs.push(inherited_attr.clone());
    }
    merged_attrs
}

/// Converts a semantic convention attribute to a resolved attribute.
pub fn resolve_attribute(
    registry: &SemConvRegistry,
    attr: &AttributeSpec,
) -> Result<attribute::Attribute, Error> {
    match attr {
        AttributeSpec::Ref { r#ref, .. } => {
            let sem_conv_attr =
                registry
                    .attribute(r#ref)
                    .ok_or(Error::FailToResolveAttributes {
                        ids: vec![r#ref.clone()],
                        error: "Attribute ref not found in the resolved registry".to_owned(),
                    })?;
            resolve_attribute(registry, sem_conv_attr)
        }
        AttributeSpec::Id {
            id,
            r#type,
            brief,
            examples,
            tag,
            requirement_level,
            sampling_relevant,
            note,
            stability,
            deprecated,
        } => Ok(attribute::Attribute {
            name: id.clone(),
            r#type: r#type.clone(),
            brief: brief.clone().unwrap_or_default(),
            examples: examples.clone(),
            tag: tag.clone(),
            requirement_level: requirement_level.clone(),
            sampling_relevant: *sampling_relevant,
            note: note.clone(),
            stability: stability.clone(),
            deprecated: deprecated.clone(),
            tags: None,
            value: None,
        }),
    }
}

#[allow(dead_code)] // ToDo Remove this once we have values in the resolved schema
fn semconv_to_resolved_value(
    value: &Option<ValueSpec>,
) -> Option<weaver_resolved_schema::value::Value> {
    value.as_ref().map(|value| match value {
        ValueSpec::String(s) => weaver_resolved_schema::value::Value::String { value: s.clone() },
        ValueSpec::Int(i) => weaver_resolved_schema::value::Value::Int { value: *i },
        ValueSpec::Double(d) => weaver_resolved_schema::value::Value::Double { value: *d },
    })
}
