use crate::{challenge::ChallengeStateValue, primitives::*};
use serde::{Deserialize, Serialize};

// VERIFY MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyManifestsInput {
    pub seed: Seed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyManifestsOutput {
    pub storer: Account,
    pub valid_manifests: Vec<Cid>,
    pub invalid_manifests: Vec<Cid>,
}

// UPLOAD MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestInput {
    pub seed: Seed,
    pub manifest_metadata: serde_json::Value,
    pub cid: Cid,
    pub pool_id: PoolId,
    pub replication_factor: ReplicationFactor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestOutput {
    pub uploader: Account,
    pub storers: Vec<Account>,
    pub manifest_metadata: serde_json::Value,
    pub pool_id: PoolId,
}

// BATCH UPLOAD MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchUploadManifestInput {
    pub seed: Seed,
    pub manifest_metadata: Vec<serde_json::Value>,
    pub cid: Vec<Cid>,
    pub pool_id: Vec<PoolId>,
    pub replication_factor: Vec<ReplicationFactor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchUploadManifestOutput {
    pub uploader: Account,
    pub pool_id: Vec<PoolId>,
    pub manifest_metadata: Vec<serde_json::Value>,
}

//UPDATE MANIFESTS STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub cid: Cid,
    pub pool_id: PoolId,
    pub active_days: i32,
    pub active_cycles: u16,
    pub missed_cycles: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatedManifestOutput {
    pub storer: Account,
    pub pool_id: PoolId,
    pub cid: Cid,
    pub active_days: i32,
    pub active_cycles: u16,
    pub missed_cycles: u16,
}

// STORAGE MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageManifestInput {
    pub seed: Seed,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageManifestOutput {
    pub storer: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

// BATCH STORAGE MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchStorageManifestInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub cid: Vec<Cid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchStorageManifestOutput {
    pub storer: Account,
    pub pool_id: PoolId,
    pub cid: Vec<Cid>,
}

// REMOVE MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestInput {
    pub seed: Seed,
    pub cid: Cid,
    pub pool_id: PoolId,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestOutput {
    pub uploader: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

// BATCH REMOVE MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRemoveManifestInput {
    pub seed: Seed,
    pub pool_id: Vec<PoolId>,
    pub cid: Vec<Cid>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRemoveManifestOutput {
    pub uploader: Account,
    pub pool_id: Vec<PoolId>,
    pub cid: Vec<Cid>,
}

// REMOVE STORING A MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStoringManifestInput {
    pub seed: Seed,
    pub cid: Cid,
    pub pool_id: PoolId,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStoringManifestOutput {
    pub storer: Option<Account>,
    pub cid: Cid,
    pub pool_id: PoolId,
}

// BATCH REMOVE STORING A MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRemoveStoringManifestInput {
    pub seed: Seed,
    pub pool_id: PoolId,
    pub cid: Vec<Cid>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRemoveStoringManifestOutput {
    pub storer: Account,
    pub pool_id: PoolId,
    pub cid: Vec<Cid>,
}

// GET MANIFESTS STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsInput {
    pub pool_id: Option<PoolId>,
    pub uploader: Option<Account>,
    pub storer: Option<Account>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsOutput {
    pub manifests: Vec<Manifest>,
}

// GET MANIFESTS STORER DATA STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsStorerDataInput {
    pub pool_id: Option<PoolId>,
    pub storer: Option<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsStorerDataOutput {
    pub manifests: Vec<ManifestStorageData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestStorageData {
    pub pool_id: PoolId,
    pub account: Account,
    pub cid: Cid,
    pub active_days: i32,
    pub active_cycles: u16,
    pub missed_cycles: u16,
    pub state: ChallengeStateValue,
}

// GET AVAILABLE MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvailableManifestsInput {
    pub pool_id: Option<PoolId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvailableManifestsOutput {
    pub manifests: Vec<ManifestAvailable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestAvailable {
    pub pool_id: PoolId,
    pub manifest_metadata: serde_json::Value,
    pub replication_available: ReplicationFactor,
}

// GENERAL MANIFEST STRUCTS

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub pool_id: PoolId,
    pub uploaders: Vec<UploaderData>,
    pub manifest_metadata: serde_json::Value,
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploaderData {
    pub uploader: Account,
    pub storers: Vec<Account>,
    pub replication_available: ReplicationFactor,
}
