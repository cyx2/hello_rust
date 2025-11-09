use mongodb::bson::{doc, Bson, Document};
use mongodb::options::{
    DeleteOptions, FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions,
    ReplaceOptions, UpdateOptions,
};
use serde::{Deserialize, Serialize};

fn empty_document() -> Document {
    doc! {}
}

#[derive(Debug, Deserialize)]
pub struct NamespacePayload {
    pub database: String,
    pub collection: String,
}

#[derive(Debug, Deserialize)]
pub struct InsertOneRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    pub document: Document,
    #[serde(default)]
    pub options: Option<InsertOneOptions>,
}

#[derive(Debug, Serialize)]
pub struct InsertOneResponse {
    pub inserted_id: Bson,
}

#[derive(Debug, Deserialize)]
pub struct InsertManyRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    pub documents: Vec<Document>,
    #[serde(default)]
    pub options: Option<InsertManyOptions>,
}

#[derive(Debug, Serialize)]
pub struct InsertManyResponse {
    pub inserted_ids: Vec<Bson>,
}

#[derive(Debug, Deserialize)]
pub struct FindOneRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    #[serde(default = "empty_document")]
    pub filter: Document,
    #[serde(default)]
    pub options: Option<FindOneOptions>,
}

#[derive(Debug, Serialize)]
pub struct FindOneResponse {
    pub document: Document,
}

#[derive(Debug, Deserialize)]
pub struct FindManyRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    #[serde(default = "empty_document")]
    pub filter: Document,
    #[serde(default)]
    pub options: Option<FindOptions>,
}

#[derive(Debug, Serialize)]
pub struct FindManyResponse {
    pub documents: Vec<Document>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    pub filter: Document,
    pub update: Document,
    #[serde(default)]
    pub options: Option<UpdateOptions>,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub matched_count: u64,
    pub modified_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upserted_id: Option<Bson>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceOneRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    pub filter: Document,
    pub replacement: Document,
    #[serde(default)]
    pub options: Option<ReplaceOptions>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    #[serde(flatten)]
    pub namespace: NamespacePayload,
    pub filter: Document,
    #[serde(default)]
    pub options: Option<DeleteOptions>,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub deleted_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct CollectionQuery {
    pub database: String,
}

#[derive(Debug, Serialize)]
pub struct CollectionsResponse {
    pub collections: Vec<String>,
}

impl UpdateResponse {
    pub fn from_update_result(result: mongodb::results::UpdateResult) -> Self {
        Self {
            matched_count: result.matched_count,
            modified_count: result.modified_count,
            upserted_id: result.upserted_id,
        }
    }
}

impl InsertManyResponse {
    pub fn from_result(result: mongodb::results::InsertManyResult) -> Self {
        let mut ids: Vec<(usize, Bson)> = result.inserted_ids.into_iter().collect();
        ids.sort_by_key(|(index, _)| *index);
        Self {
            inserted_ids: ids.into_iter().map(|(_, id)| id).collect(),
        }
    }
}
