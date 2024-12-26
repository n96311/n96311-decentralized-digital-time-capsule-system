#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define memory and id cell types
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
// Content types that can be stored in the time capsule
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum CapsuleContent {
    Text(String),
    EncryptedMessage {
        content: Vec<u8>,
        public_key: String,
    },
    MediaReference {
        ipfs_hash: String,
        media_type: String,
    },
    MultipartMessage {
        parts: Vec<CapsuleContent>,
        title: String,
    },
}

// Access control for the capsule
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum AccessControl {
    Public,
    Private {
        allowed_viewers: Vec<String>, // Principal IDs
    },
    Conditional {
        condition_type: String,
        condition_data: String, // Could be a smart contract address, oracle reference, etc.
    },
}

// Main time capsule structure
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct TimeCapsule {
    id: u64,
    creator: String, // Principal ID
    creation_date: u64,
    unlock_date: u64,
    content: CapsuleContent,
    access_control: AccessControl,
    metadata: CapsuleMetadata,
    status: CapsuleStatus,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct CapsuleMetadata {
    title: String,
    description: String,
    tags: Vec<String>,
    location: Option<GeoLocation>,
    cultural_significance: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    location_name: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum CapsuleStatus {
    Sealed,
    UnlockPending,
    Unlocked,
    Archived,
}

// Payload for creating a new time capsule
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct CreateCapsulePayload {
    content: CapsuleContent,
    unlock_date: u64,
    access_control: AccessControl,
    metadata: CapsuleMetadata,
}

// Storage implementation
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static CAPSULE_STORAGE: RefCell<StableBTreeMap<u64, TimeCapsule, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static ID_COUNTER: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create counter")
    );
}

// Implementation for TimeCapsule
impl Storable for TimeCapsule {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TimeCapsule {
    const MAX_SIZE: u32 = 1024 * 1024; // 1MB max size
    const IS_FIXED_SIZE: bool = false;
}

// Create a new time capsule
#[ic_cdk::update]
fn create_time_capsule(payload: CreateCapsulePayload) -> Result<TimeCapsule, String> {
    let caller = ic_cdk::caller().to_string();
    let current_time = time();
    
    if payload.unlock_date <= current_time {
        return Err("Unlock date must be in the future".to_string());
    }

    let capsule_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
            .expect("Failed to increment counter");
        current_value
    });

    let capsule = TimeCapsule {
        id: capsule_id,
        creator: caller,
        creation_date: current_time,
        unlock_date: payload.unlock_date,
        content: payload.content,
        access_control: payload.access_control,
        metadata: payload.metadata,
        status: CapsuleStatus::Sealed,
    };

    CAPSULE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(capsule_id, capsule.clone());
    });

    Ok(capsule)
}

// Retrieve a time capsule if conditions are met
#[ic_cdk::query]
fn get_capsule(capsule_id: u64) -> Result<TimeCapsule, String> {
    let caller = ic_cdk::caller().to_string();
    let current_time = time();

    CAPSULE_STORAGE.with(|storage| {
        if let Some(capsule) = storage.borrow().get(&capsule_id) {
            // Check if capsule is unlockable
            if current_time < capsule.unlock_date {
                return Err("Capsule is still sealed".to_string());
            }

            // Check access control
            match &capsule.access_control {
                AccessControl::Public => Ok(capsule),
                AccessControl::Private { allowed_viewers } => {
                    if allowed_viewers.contains(&caller) || capsule.creator == caller {
                        Ok(capsule)
                    } else {
                        Err("Access denied".to_string())
                    }
                }
                AccessControl::Conditional { condition_type, condition_data } => {
                    // Implement condition checking logic
                    validate_condition(condition_type, condition_data, &caller)
                        .map(|_| capsule)
                }
            }
        } else {
            Err("Capsule not found".to_string())
        }
    })
}

// Function to validate conditional access
fn validate_condition(condition_type: &str, condition_data: &str, caller: &str) -> Result<(), String> {
    match condition_type {
        "token_holder" => {
            // Token holding verification
            Ok(())
        }
        "geo_location" => {
            // Location verification
            Ok(())
        }
        "quiz" => {
            // Quiz verification
            Ok(())
        }
        _ => Err("Unknown condition type".to_string()),
    }
}

// Get all public capsules that are unlocked
#[ic_cdk::query]
fn get_public_capsules() -> Vec<TimeCapsule> {
    let current_time = time();
    
    CAPSULE_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, capsule)| {
                matches!(capsule.access_control, AccessControl::Public) && 
                current_time >= capsule.unlock_date
            })
            .map(|(_, capsule)| capsule)
            .collect()
    })
}

// Get capsules by location
#[ic_cdk::query]
fn get_capsules_by_location(latitude: f64, longitude: f64, radius_km: f64) -> Vec<TimeCapsule> {
    CAPSULE_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, capsule)| {
                if let Some(location) = &capsule.metadata.location {
                    calculate_distance(
                        latitude, longitude,
                        location.latitude, location.longitude
                    ) <= radius_km
                } else {
                    false
                }
            })
            .map(|(_, capsule)| capsule)
            .collect()
    })
}

// Helper function to calculate distance between two points
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Haversine formula implementation
    const R: f64 = 6371.0; // Earth's radius in kilometers
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2) +
        lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    R * c
}

// Export Candid interface
ic_cdk::export_candid!();