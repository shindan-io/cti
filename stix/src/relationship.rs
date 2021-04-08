use std::borrow::Cow;

use petgraph::EdgeDirection;
use serde::Deserialize;

use crate::{CommonProperties, Id, TypedObject};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RelationshipType {
    Mitigates,
    Uses,
    RevokedBy,
    SubtechniqueOf,
}

#[derive(Deserialize)]
pub struct Relationship {
    #[serde(flatten)]
    base: CommonProperties,
    pub source_ref: Id,
    pub target_ref: Id,
    pub relationship_type: RelationshipType,
}

impl TypedObject for Relationship {
    const TYPE: &'static str = "relationship";
}

impl AsRef<CommonProperties> for Relationship {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}

pub(crate) struct Filter {
    pub direction: EdgeDirection,
    pub relationship_type: RelationshipType,
    pub peer_type: Cow<'static, str>,
}

impl Filter {
    pub fn outgoing<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
        Self {
            direction: EdgeDirection::Outgoing,
            relationship_type,
            peer_type: Cow::Borrowed(Peer::TYPE),
        }
    }

    pub fn incoming<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
        Filter {
            direction: EdgeDirection::Incoming,
            relationship_type,
            peer_type: Cow::Borrowed(Peer::TYPE),
        }
    }
}

impl PartialEq<Filter> for Relationship {
    fn eq(&self, other: &Filter) -> bool {
        let peer = match other.direction {
            EdgeDirection::Outgoing => &self.target_ref,
            EdgeDirection::Incoming => &self.source_ref,
        };

        peer.object_type() == other.peer_type
    }
}
