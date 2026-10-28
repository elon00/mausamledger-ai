use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MinistryState {
    pub ministry_id: u32,
    pub ministry_name: String,
    pub project_name: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditRecord {
    pub record_id: String,
    pub timestamp: u64,
    pub state_hash: String,
    pub verified: bool,
}

pub fn get_metadata() -> MinistryState {
    MinistryState {
        ministry_id: 15,
        ministry_name: "Ministry of Earth Sciences".to_string(),
        project_name: "MausamLedger AI".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Real-Time Climate Sensor Network".to_string(),
        target_chain: "Internet Computer (ICP) + Solana".to_string(),
        is_active: true,
    }
}
