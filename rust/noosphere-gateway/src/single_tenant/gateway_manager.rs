use std::sync::Arc;

use crate::{
    extractors::GatewayScope, single_tenant::SingleTenantJobClient, GatewayManager,
    SingleTenantContextResolver, SphereContextResolver,
};
use anyhow::Result;
use async_trait::async_trait;
use axum::http::{request::Parts, StatusCode};
use noosphere_core::context::HasMutableSphereContext;
use noosphere_core::data::Did;
use noosphere_ipfs::KuboClient;
use noosphere_storage::{Storage, UcanStore};
use url::Url;

/// Implements [GatewayManager] for a single sphere context, used in the single-tenant
/// gateway workflow in `orb`.
#[derive(Clone)]
pub struct SingleTenantGatewayManager<C, S>
where
    C: HasMutableSphereContext<S> + 'static,
    S: Storage + 'static,
{
    context: C,
    gateway_scope: GatewayScope<C, S>,
    context_resolver: SingleTenantContextResolver<C, S>,
    job_client: Arc<SingleTenantJobClient<C, S>>,
    ipfs_api: Url,
    cors_origin: Option<Url>,
    marker: std::marker::PhantomData<S>,
}

impl<C, S> SingleTenantGatewayManager<C, S>
where
    C: HasMutableSphereContext<S> + 'static,
    S: Storage + 'static,
{
    /// Create a new [SingleTenantGatewayManager], implementing [GatewayManager] for a single sphere `context`.
    pub async fn new(
        context: C,
        counterpart: Did,
        ipfs_api: Url,
        name_resolver_api: Url,
        cors_origin: Option<Url>,
    ) -> Result<Self> {
        let (gateway, gateway_identity) = {
            let ctx = context.sphere_context().await?;
            (ctx.identity().to_owned(), ctx.author().did().await?)
        };
        let gateway_scope = GatewayScope::new(gateway_identity, gateway, counterpart);
        let context_resolver =
            SingleTenantContextResolver::new(context.clone(), gateway_scope.clone());
        let job_client = Arc::new(
            SingleTenantJobClient::new(
                context_resolver.clone(),
                gateway_scope.clone(),
                KuboClient::new(&ipfs_api)?,
                name_resolver_api,
            )
            .await?,
        );
        Ok(SingleTenantGatewayManager {
            context,
            gateway_scope,
            context_resolver,
            job_client,
            ipfs_api,
            cors_origin,
            marker: std::marker::PhantomData,
        })
    }
}

#[async_trait]
impl<C, S> GatewayManager<C, S> for SingleTenantGatewayManager<C, S>
where
    C: HasMutableSphereContext<S>,
    S: Storage + 'static,
{
    type JobClient = Arc<SingleTenantJobClient<C, S>>;

    fn job_client(&self) -> Self::JobClient {
        self.job_client.clone()
    }

    fn ipfs_api_url(&self) -> Url {
        self.ipfs_api.to_owned()
    }

    fn cors_origin(&self) -> Option<Url> {
        self.cors_origin.to_owned()
    }

    async fn ucan_store(&self, sphere_identity: &Did) -> Result<UcanStore<S::BlockStore>> {
        match &self.gateway_scope.gateway == sphere_identity {
            true => {
                let context = self.context.sphere_context().await?;
                let db = context.db().to_block_store();
                Ok(UcanStore(db))
            }
            false => Err(anyhow::anyhow!(
                "No ucan store found with identity: {sphere_identity}."
            )),
        }
    }

    async fn sphere_context(&self, sphere_identity: &Did) -> Result<C> {
        self.context_resolver.get_context(sphere_identity).await
    }

    async fn gateway_scope(&self, _: &mut Parts) -> Result<(Did, Did, Did), StatusCode> {
        Ok((
            self.gateway_scope.gateway_identity.clone(),
            self.gateway_scope.gateway.clone(),
            self.gateway_scope.counterpart.clone(),
        ))
    }
}
