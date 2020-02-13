use futures::prelude::*;
use rlay_plugin_interface::prelude::*;
use rustc_hex::FromHex;
use serde::Deserialize;
use std::collections::HashMap;

#[no_mangle]
extern "C" fn init_filter_plugin() -> Box<dyn RlayFilter + Send + Sync> {
    Box::new(VersionFilter)
}

pub struct VersionFilter;

#[derive(Deserialize, Clone)]
pub struct FilterParams {
    version_property: String,
    #[serde(default = "FilterParams::default_keep_unversioned")]
    keep_unversioned: bool,
}

impl FilterParams {
    pub fn default_keep_unversioned() -> bool {
        false
    }

    pub fn version_property(&self) -> Vec<u8> {
        self.version_property[2..].from_hex().unwrap()
    }
}

impl RlayFilter for VersionFilter {
    fn filter_name(&self) -> &'static str {
        "version"
    }

    fn filter_entities<'a>(
        &self,
        ctx: &'a FilterContext<'a>,
        entities: Vec<Entity>,
    ) -> BoxFuture<'a, Vec<bool>> {
        let params: FilterParams = serde_json::from_value(ctx.params.clone()).unwrap();

        let filter_markers = async move {
            // Vec<(entity, needs_filtering, version)>
            let entities_with_versions =
                Self::entities_with_versions(ctx, params.clone(), entities).await;
            let highest_version_per_property =
                Self::highest_version_per_property(&entities_with_versions);

            entities_with_versions
                .into_iter()
                .map(|(entity, needs_filtering, version)| {
                    if !needs_filtering {
                        return true;
                    }

                    match version {
                        None => params.keep_unversioned,
                        Some(version) => {
                            let property = match entity {
                                Entity::DataPropertyAssertion(inner) => inner.property.clone(),
                                Entity::ObjectPropertyAssertion(inner) => inner.property.clone(),
                                _ => panic!("This entity kind should not be filtered"),
                            };
                            // version of entity is equal to highest observed version
                            version
                                == highest_version_per_property
                                    .get(&property.unwrap())
                                    .unwrap()
                                    .unwrap()
                        }
                    }
                })
                .collect::<Vec<_>>()
        };

        Box::pin(filter_markers)
    }
}

impl VersionFilter {
    pub fn entity_needs_filtering(entity: &Entity) -> bool {
        match entity {
            Entity::DataPropertyAssertion(_) => true,
            Entity::ObjectPropertyAssertion(_) => true,
            _ => false,
        }
    }

    /// Create a mapping of Data-/ObjectProperties to their respective highest observed versions
    pub fn highest_version_per_property(
        entities_with_versions: &[(Entity, bool, Option<u64>)],
    ) -> HashMap<Vec<u8>, Option<u64>> {
        let mut version_map: HashMap<Vec<u8>, Option<u64>> = HashMap::new();
        for (entity, needs_filtering, version) in entities_with_versions {
            if !needs_filtering {
                continue;
            }

            let property = match entity {
                Entity::DataPropertyAssertion(inner) => inner.property.clone(),
                Entity::ObjectPropertyAssertion(inner) => inner.property.clone(),
                _ => panic!("This entity kind should not be filtered"),
            };

            version_map
                .entry(property.unwrap())
                .and_modify(|highest_version| {
                    if version > highest_version {
                        *highest_version = version.clone();
                    }
                })
                .or_default();
        }

        version_map
    }

    pub async fn entities_with_versions<'a>(
        ctx: &'a FilterContext<'a>,
        params: FilterParams,
        entities: Vec<Entity>,
    ) -> Vec<(Entity, bool, Option<u64>)> {
        stream::iter(entities)
            .then(|entity| {
                async {
                    match Self::entity_needs_filtering(&entity) {
                        false => (entity, false, None),
                        true => (
                            entity.clone(),
                            true,
                            Self::get_version_number(ctx, params.clone(), entity).await,
                        ),
                    }
                }
            })
            .collect::<Vec<_>>()
            .await
    }

    /// Get annotation matching the versioning annotation property and extract its value
    pub async fn get_version_number<'a>(
        ctx: &'a FilterContext<'a>,
        params: FilterParams,
        entity: Entity,
    ) -> Option<u64> {
        let raw_annotations: Vec<Vec<u8>> = match entity {
            Entity::DataPropertyAssertion(inner) => inner.annotations.clone(),
            Entity::ObjectPropertyAssertion(inner) => inner.annotations.clone(),
            _ => return None,
        };

        let mut version_annotation = None;
        for raw_annotation in raw_annotations {
            let annotation = ctx
                .backend
                .get_entity(&raw_annotation)
                .await
                .unwrap()
                .unwrap();
            let annotation = match annotation {
                Entity::Annotation(inner) => inner,
                _ => continue,
            };
            if annotation.property != params.version_property() {
                continue;
            }
            version_annotation = Some(annotation);
        }
        let version_annotation = match version_annotation {
            None => return None,
            Some(inner) => inner,
        };

        Some(serde_cbor::from_slice(&version_annotation.value).unwrap())
    }
}
