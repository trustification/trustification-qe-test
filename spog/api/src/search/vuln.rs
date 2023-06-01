use std::collections::HashMap;

use super::{QueryParams, SharedState};
use actix_web::{web, HttpResponse, Responder};
use spog_model::search::{SearchResult, VulnSummary};
use trustification_index::IndexStore;
use vexination_model::prelude::*;

const MAX_LIMIT: usize = 1_000;

pub async fn search(state: web::Data<SharedState>, params: web::Query<QueryParams>) -> impl Responder {
    let params = params.into_inner();
    tracing::trace!("Querying VEX using {}", params.q);
    let state = &state.vex;

    let index = state.index.read().await;
    let result = search_vex(&index, &params.q, params.offset, params.limit.min(MAX_LIMIT)).await;

    let mut result = match result {
        Err(e) => {
            tracing::info!("Error searching: {:?}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(result) => result,
    };

    // Deduplicate data
    let mut m: HashMap<String, VulnSummary> = HashMap::new();
    for item in result.drain(..) {
        if let Some(entry) = m.get_mut(&item.cve) {
            entry.advisories.push(item.advisory);
        } else {
            m.insert(
                item.cve.clone(),
                VulnSummary {
                    cve: item.cve,
                    advisories: vec![item.advisory],
                    title: item.title,
                    description: item.description,
                    release: item.release,
                    cvss: item.cvss,
                    affected_packages: item.affected_packages,
                },
            );
        }
    }

    HttpResponse::Ok().json(SearchResult::<Vec<VulnSummary>> {
        total: result.total,
        result: m.values().map(|v| v.clone()).collect(),
    })
}

async fn search_vex(
    index: &IndexStore<vexination_index::Index>,
    q: &str,
    offset: usize,
    limit: usize,
) -> anyhow::Result<SearchResult<Vec<SearchDocument>>> {
    Ok(index.search(q, offset, limit)?.into())
}