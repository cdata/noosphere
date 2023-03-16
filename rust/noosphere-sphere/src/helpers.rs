// #![cfg(any(test, doc, feature = "helpers"))]

use std::sync::Arc;

use anyhow::Result;
use noosphere_core::{
    authority::{generate_ed25519_key, Author},
    view::Sphere,
};
use noosphere_storage::{MemoryStorage, SphereDb, TrackingStorage};
use tokio::sync::Mutex;
use ucan::crypto::KeyMaterial;
use ucan_key_support::ed25519::Ed25519KeyMaterial;

use crate::SphereContext;

/// Access levels available when simulating a [SphereContext]
pub enum SimulationAccess {
    Readonly,
    ReadWrite,
}

/// Create a temporary, non-persisted [SphereContext] that tracks usage
/// internally. This is intended for use in docs and tests, and should otherwise
/// be ignored. When creating the simulated [SphereContext], you can pass a
/// [SimulationAccess] to control the kind of access the emphemeral credentials
/// have to the [SphereContext].
pub async fn simulated_sphere_context(
    profile: SimulationAccess,
) -> Result<Arc<Mutex<SphereContext<Ed25519KeyMaterial, TrackingStorage<MemoryStorage>>>>> {
    let storage_provider = TrackingStorage::wrap(MemoryStorage::default());
    let mut db = SphereDb::new(&storage_provider).await?;

    let owner_key = generate_ed25519_key();
    let owner_did = owner_key.get_did().await?;

    let (sphere, proof, _) = Sphere::generate(&owner_did, &mut db).await?;

    let sphere_identity = sphere.get_identity().await.unwrap();
    let author = Author {
        key: owner_key,
        authorization: match profile {
            SimulationAccess::Readonly => None,
            SimulationAccess::ReadWrite => Some(proof),
        },
    };

    db.set_version(&sphere_identity, sphere.cid()).await?;

    Ok(Arc::new(Mutex::new(
        SphereContext::new(sphere_identity, author, db)
            .await
            .unwrap(),
    )))
}