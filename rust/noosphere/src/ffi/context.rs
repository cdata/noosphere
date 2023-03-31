// See: https://github.com/getditto/safer_ffi/issues/31#issuecomment-782070270
#![allow(improper_ctypes_definitions)]

use anyhow::anyhow;
use cid::Cid;
use itertools::Itertools;
use noosphere_core::data::Did;
use safer_ffi::{char_p::InvalidNulTerminator, prelude::*};
use std::{pin::Pin, str::FromStr, sync::Arc};
use subtext::{Peer, Slashlink};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    sync::Mutex,
};

use crate::{
    ffi::{NsError, NsHeaders, NsNoosphere, TryOrInitialize},
    platform::{PlatformKeyMaterial, PlatformStorage},
};

use noosphere_sphere::{
    HasMutableSphereContext, HasSphereContext, SphereContentRead, SphereContentWrite,
    SphereContext, SphereCursor, SphereFile, SphereWalker,
};

#[derive_ReprC(rename = "ns_sphere")]
#[repr(opaque)]
/// @class ns_sphere_t
///
/// An opaque struct representing a sphere.
pub struct NsSphere {
    inner: SphereCursor<
        Arc<Mutex<SphereContext<PlatformKeyMaterial, PlatformStorage>>>,
        PlatformKeyMaterial,
        PlatformStorage,
    >,
}

impl NsSphere {
    pub fn inner(
        &self,
    ) -> &SphereCursor<
        Arc<Mutex<SphereContext<PlatformKeyMaterial, PlatformStorage>>>,
        PlatformKeyMaterial,
        PlatformStorage,
    > {
        &self.inner
    }

    pub fn inner_mut(
        &mut self,
    ) -> &mut SphereCursor<
        Arc<Mutex<SphereContext<PlatformKeyMaterial, PlatformStorage>>>,
        PlatformKeyMaterial,
        PlatformStorage,
    > {
        &mut self.inner
    }
}

#[derive_ReprC(rename = "ns_sphere_file")]
#[repr(opaque)]
/// @class ns_sphere_file_t
/// A read/write view into a sphere's memo.
///
/// ns_sphere_file_t is a lazy, stateful view into a single memo.
/// No bytes are read from disk until ns_sphere_file_contents_read() is invoked.
pub struct NsSphereFile {
    inner: SphereFile<Pin<Box<dyn AsyncRead>>>,
}

impl NsSphereFile {
    pub fn inner(&self) -> &SphereFile<Pin<Box<dyn AsyncRead>>> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut SphereFile<Pin<Box<dyn AsyncRead>>> {
        &mut self.inner
    }
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Initialize an ns_sphere_t instance.
///
/// This will fail if it is not possible to initialize a sphere with the given
/// identity (which implies that no such sphere was ever created or joined on
/// this device).
pub fn ns_sphere_open(
    noosphere: &NsNoosphere,
    sphere_identity: char_p::Ref<'_>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> Option<repr_c::Box<NsSphere>> {
    error_out.try_or_initialize(|| {
        let fs = noosphere.async_runtime().block_on(async {
            let sphere_context = noosphere
                .inner()
                .get_sphere_context(&Did(sphere_identity.to_str().into()))
                .await?;

            let cursor = SphereCursor::latest(sphere_context);

            Ok(Box::new(NsSphere { inner: cursor }).into()) as Result<_, anyhow::Error>
        })?;

        Ok(fs)
    })
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Access another sphere by a petname.
///
/// The petname should be one that has been assigned to the sphere's identity
/// using ns_sphere_petname_set(). If any of the data required to access the
/// target sphere is not available locally, it will be replicated from the
/// network through a the configured Noosphere Gateway. If no such gateway is
/// configured and the data is not available locally, this call will fail. The
/// returned ns_sphere_t pointer can be used to access the content, petnames,
/// revision history and other features of the target sphere with the same APIs
/// used to access the local user's sphere, except that any operations that
/// attempt to modify the sphere will be rejected. Note that since this function
/// has a reasonable likelihood to call out to the network, it is possible that
/// it may block for a significant amount of time when network conditions are
/// poor.
pub fn ns_sphere_traverse_by_petname(
    noosphere: &NsNoosphere,
    sphere: &mut NsSphere,
    petname: char_p::Ref<'_>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> Option<repr_c::Box<NsSphere>> {
    error_out.try_or_initialize(|| {
        let sphere = noosphere.async_runtime().block_on(async {
            let sphere_context = sphere.inner_mut();
            let next_sphere_context = sphere_context
                .sphere_context_mut()
                .await?
                .traverse_by_petname(petname.to_str())
                .await?;

            Ok(Box::new(NsSphere {
                inner: SphereCursor::latest(Arc::new(Mutex::new(next_sphere_context))),
            })
            .into()) as Result<_, anyhow::Error>
        })?;

        Ok(sphere)
    })
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Deallocate an ns_sphere_t instance.
pub fn ns_sphere_free(sphere: repr_c::Box<NsSphere>) {
    drop(sphere)
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Read a memo as a ns_sphere_file_t from a ns_sphere_t by slashlink.
///
/// Note that although this function will eventually support slashlinks
/// that include the pet name of a peer, at this time only slashlinks
/// with slugs referencing the slug namespace of the local sphere are allowed.
///
/// This function will return a null pointer if the slug does not have a file
/// associated with it at the revision of the sphere that is referred to by the
/// ns_sphere_t being read from.
pub fn ns_sphere_content_read(
    noosphere: &NsNoosphere,
    sphere: &NsSphere,
    slashlink: char_p::Ref<'_>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> Option<repr_c::Box<NsSphereFile>> {
    match error_out.try_or_initialize(|| {
        noosphere
            .async_runtime()
            .block_on(async {
                let slashlink = match Slashlink::from_str(slashlink.to_str()) {
                    Ok(slashlink) => slashlink,
                    _ => return Ok(None),
                };

                if Peer::None != slashlink.peer {
                    return Err(anyhow!("Peer in slashlink not yet supported"));
                }

                let slug = match slashlink.slug {
                    Some(slug) => slug,
                    None => return Err(anyhow!("No slug specified in slashlink!")),
                };

                let cursor = sphere.inner();

                println!(
                    "Reading sphere {} slug {}...",
                    cursor.identity().await?,
                    slug
                );

                let file = cursor.read(&slug).await?;

                Ok(file.map(|sphere_file| {
                    Box::new(NsSphereFile {
                        inner: sphere_file.boxed(),
                    })
                    .into()
                }))
            })
            .map_err(|error| error.into())
    }) {
        Some(maybe_file) => maybe_file,
        None => None,
    }
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Write content to a ns_sphere_t instance, keyed by `slug`, assigning its
/// content-type header to the specified value.
///
/// If additional headers are specified, they will be appended to the list
/// of headers in the memo that is created for the content. If some content
/// already exists at the specified slug, it will be assigned to be the
/// previous historical revision of the new content.
///
/// Note that you must invoke ns_sphere_save() to commit one or more writes
/// to the sphere.
pub fn ns_sphere_content_write(
    noosphere: &NsNoosphere,
    sphere: &mut NsSphere,
    slug: char_p::Ref<'_>,
    content_type: char_p::Ref<'_>,
    bytes: c_slice::Ref<'_, u8>,
    additional_headers: Option<&NsHeaders>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) {
    error_out.try_or_initialize(|| {
        noosphere.async_runtime().block_on(async {
            let slug = slug.to_str();
            let cursor = sphere.inner_mut();

            println!(
                "Writing sphere {} slug {}...",
                cursor.identity().await?,
                slug
            );

            cursor
                .write(
                    slug,
                    content_type.to_str(),
                    bytes.as_ref(),
                    additional_headers.map(|headers| headers.inner().clone()),
                )
                .await?;

            println!("Updated {:?}...", slug);

            Ok(())
        })
    });
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Unlinks a slug from the content space.
///
/// Note that this does not remove the blocks that were previously associated
/// with the content found at the given slug, because they will still be
/// available at an earlier revision of the sphere. In order to commit the
/// change, you must save. Note that this call is a no-op if there is
/// no matching slug linked in the sphere.
pub fn ns_sphere_content_remove(
    noosphere: &NsNoosphere,
    sphere: &mut NsSphere,
    slug: char_p::Ref<'_>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) {
    error_out.try_or_initialize(|| {
        noosphere
            .async_runtime()
            .block_on(async { sphere.inner_mut().remove(slug.to_str()).await })?;
        Ok(())
    });
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Save any writes performed on the ns_sphere_t instance.
///
/// If additional headers are specified, they will be appended to
/// the headers in the memo that is created to wrap the latest sphere revision.
pub fn ns_sphere_save(
    noosphere: &NsNoosphere,
    sphere: &mut NsSphere,
    additional_headers: Option<&NsHeaders>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) {
    error_out.try_or_initialize(|| {
        let cid = noosphere.async_runtime().block_on(
            sphere
                .inner_mut()
                .save(additional_headers.map(|headers| headers.inner().clone())),
        )?;

        println!("Saved sphere; new revision is {}", cid);

        Ok(())
    });
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Get an array of all of the slugs in a sphere at the current version.
pub fn ns_sphere_content_list(
    noosphere: &NsNoosphere,
    sphere: &NsSphere,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> c_slice::Box<char_p::Box> {
    let possible_output = error_out.try_or_initialize(|| {
        noosphere.async_runtime().block_on(async {
            let slug_set = SphereWalker::from(sphere.inner()).list_slugs().await?;
            let mut all_slugs: Vec<char_p::Box> = Vec::new();

            for slug in slug_set.into_iter() {
                all_slugs.push(
                    slug.try_into()
                        .map_err(|error: InvalidNulTerminator<String>| anyhow!(error))?,
                );
            }

            Ok(all_slugs)
        })
    });

    match possible_output {
        Some(slugs) => slugs,
        None => Vec::new(),
    }
    .into_boxed_slice()
    .into()
}

#[ffi_export]
/// @memberof ns_sphere_t
/// Get an array of all of the slugs that changed in a given sphere since a
/// given revision of that sphere (excluding the given revision).
///
/// The revision should be provided as a base64-encoded CID v1 string.
/// If no revision is provided, the entire history will be considered,
/// back to and including the first revision.
///
/// Note that a slug change may mean the slug was added, updated or removed.
/// Also note that multiple changes to the same slug will be reduced to a
/// single entry in the array that is returned.
pub fn ns_sphere_content_changes(
    noosphere: &NsNoosphere,
    sphere: &NsSphere,
    since_cid: Option<char_p::Ref<'_>>,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> c_slice::Box<char_p::Box> {
    let possible_output = error_out.try_or_initialize(|| {
        noosphere.async_runtime().block_on(async {
            let since = match since_cid {
                Some(cid_string) => {
                    Some(Cid::from_str(cid_string.to_str()).map_err(|error| anyhow!(error))?)
                }
                None => None,
            };

            let changed_slug_set = SphereWalker::from(sphere.inner())
                .content_changes(since.as_ref())
                .await?;
            let mut changed_slugs: Vec<char_p::Box> = Vec::new();

            for slug in changed_slug_set.into_iter() {
                changed_slugs.push(
                    slug.try_into()
                        .map_err(|error: InvalidNulTerminator<String>| anyhow!(error))?,
                );
            }

            Ok(changed_slugs)
        })
    });

    match possible_output {
        Some(slugs) => slugs,
        None => Vec::new(),
    }
    .into_boxed_slice()
    .into()
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Deallocate a ns_sphere_file_t instance.
pub fn ns_sphere_file_free(sphere_file: repr_c::Box<NsSphereFile>) {
    drop(sphere_file)
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Read the contents of an ns_sphere_file_t as a byte array.
///
/// Bytes can be read from a ns_sphere_file_t instance only once.
pub fn ns_sphere_file_contents_read(
    noosphere: &NsNoosphere,
    sphere_file: &mut NsSphereFile,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> Option<c_slice::Box<u8>> {
    error_out.try_or_initialize(|| {
        noosphere.async_runtime().block_on(async {
            let mut buffer = Vec::new();

            sphere_file
                .inner_mut()
                .contents
                .read_to_end(&mut buffer)
                .await
                .map_err(|error| anyhow!(error))?;

            Ok(buffer.into_boxed_slice().into())
        })
    })
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Read all header values for a file that correspond to a given name, returning
/// them as an array of strings
pub fn ns_sphere_file_header_values_read(
    sphere_file: &NsSphereFile,
    name: char_p::Ref<'_>,
) -> c_slice::Box<char_p::Box> {
    sphere_file
        .inner
        .memo
        .get_header(name.to_str())
        .into_iter()
        .filter_map(|header| header.try_into().ok())
        .collect::<Vec<char_p::Box>>()
        .into_boxed_slice()
        .into()
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Get the first header value for a given name in the file, if any.
pub fn ns_sphere_file_header_value_first(
    sphere_file: &NsSphereFile,
    name: char_p::Ref<'_>,
) -> Option<char_p::Box> {
    sphere_file
        .inner
        .memo
        .get_first_header(name.to_str())
        .into_iter()
        .filter_map(|value| value.try_into().ok())
        .next()
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Read all the headers associated with a file as an array of strings.
///
/// The headers will be reduced to a single entry in cases where multiple
/// headers share the same name.
pub fn ns_sphere_file_header_names_read(sphere_file: &NsSphereFile) -> c_slice::Box<char_p::Box> {
    sphere_file
        .inner
        .memo
        .headers
        .iter()
        .map(|(name, _)| name)
        .unique()
        .filter_map(|name| name.to_owned().try_into().ok())
        .collect::<Vec<char_p::Box>>()
        .into_boxed_slice()
        .into()
}

#[ffi_export]
/// @memberof ns_sphere_file_t
/// Get the base64-encoded CID v1 string for the memo that refers to the content
/// of this ns_sphere_file_t.
pub fn ns_sphere_file_version_get(
    sphere_file: &NsSphereFile,
    error_out: Option<Out<'_, repr_c::Box<NsError>>>,
) -> Option<char_p::Box> {
    error_out.try_or_initialize(|| {
        sphere_file
            .inner
            .memo_version
            .to_string()
            .try_into()
            .map_err(|error: InvalidNulTerminator<String>| anyhow!(error).into())
    })
}