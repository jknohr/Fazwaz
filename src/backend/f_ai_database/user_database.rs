
use tracing::{info, warn, error, instrument};
use serde::{Deserialize, Serialize};
use crate::backend::f_ai_database::{DatabaseManager, EventLogger, MetricsCollector};
use chrono::{DateTime, Utc};
use uuid7::Uuid7;

#[derive(Debug, Serialize, Deserialize)]
pubstruct Version {
    major: u32,
    minor: u32,
    patch: u32,
    timestamp: DateTime<Utc>,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: Uuid7,
    email: String,
    secondary_email: Option<String>,
    main_phone_number: String,
    secondary_phone_number: Option<String>,
    nationality: String,
    speaking_languages: Vec<String>,
    created_at: DateTime<Utc>,
    last_login_at: DateTime<Utc>,
    version: Version,
    profile: UserProfile,
    social_profiles: Vec<SocialProfile>,
    client_details: Option<ClientDetails>,
    listings: Vec<String>, // listing IDs
    api_keys: Vec<ApiKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    first_name: String,
    last_name: String,
    gender: String,
    date_of_birth: DateTime<Utc>,
    created_at: DateTime<Utc>,
    version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProfile {
    platform: String,
    username: String,
    created_at: DateTime<Utc>,
    version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDetails {
    budget: String,
    preferences: Vec<String>,
    family_status: String,
    income: f64,
    property_preferences: Vec<String>,
    searching_for: String,
    occupation: String,
    price_range: PriceRange,
    characteristics: Vec<String>,
    created_at: DateTime<Utc>,
    version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceRange {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    key: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientPreferences {
    preference_type: String,
    value: String,
    version: Version,
}

use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use anyhow::Result;
use surrealdb::sql::Object;

#[derive(Debug)]
struct UserDatabase {
    db: Arc<DatabaseManager>,
    event_logger: Arc<EventLogger>,
    metrics: Arc<MetricsCollector>,
}

impl UserDatabase {
    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> Result<()> {
        let transaction = r#"
            BEGIN TRANSACTION;
            
            // Create base user with version tracking
            LET $user = (
                CREATE user CONTENT {
                    user_id: $user_id,
                    email: $email,
                    secondary_email: $secondary_email,
                    main_phone_number: $main_phone_number,
                    secondary_phone_number: $secondary_phone_number,
                    nationality: $nationality,
                    speaking_languages: $speaking_languages,
                    created_at: time::now(),
                    last_login_at: time::now(),
                    version: $version
                }
            );

            // Create and relate profile
            LET $profile = (
                CREATE user_profile CONTENT {
                    first_name: $first_name,
                    last_name: $last_name,
                    gender: $gender,
                    date_of_birth: $date_of_birth,
                    created_at: time::now(),
                    version: $version
                }
            );
            
            RELATE $user->has_profile->$profile CONTENT { 
                created_at: time::now(),
                version: $version
            };

            // Create and relate social profiles
            FOR $social IN $social_profiles {
                LET $new_social = (
                    CREATE social_profile CONTENT {
                        platform: $social.platform,
                        username: $social.username,
                        created_at: time::now(),
                        version: $version
                    }
                );
                
                RELATE $user->has_social->$new_social CONTENT {
                    created_at: time::now(),
                    version: $version
                };
            };

            // Create and relate client details if present
            IF $has_client_details {
                LET $details = (
                    CREATE client_details CONTENT {
                        budget: $client_details.budget,
                        preferences: $client_details.preferences,
                        family_status: $client_details.family_status,
                        income: $client_details.income,
                        property_preferences: $client_details.property_preferences,
                        searching_for: $client_details.searching_for,
                        occupation: $client_details.occupation,
                        price_range: $client_details.price_range,
                        characteristics: $client_details.characteristics,
                        created_at: time::now(),
                        version: $version
                    }
                );
                
                RELATE $user->has_details->$details CONTENT {
                    created_at: time::now(),
                    version: $version
                };
            };

            // Create and relate API keys
            FOR $key IN $api_keys {
                LET $new_key = (
                    CREATE api_key CONTENT {
                        key: $key.key,
                        created_at: time::now(),
                        expires_at: $key.expires_at,
                        version: $version
                    }
                );
                
                RELATE $user->has_api_key->$new_key CONTENT {
                    created_at: time::now(),
                    version: $version
                };
            };

            COMMIT TRANSACTION;
        "#;

        let params = Object::from([
            ("user_id", Value::from(&user.user_id)),
            ("email", Value::from(&user.email)),
            ("secondary_email", Value::from(&user.secondary_email)),
            ("main_phone_number", Value::from(&user.main_phone_number)),
            ("secondary_phone_number", Value::from(&user.secondary_phone_number)),
            ("nationality", Value::from(&user.nationality)),
            ("speaking_languages", Value::from(&user.speaking_languages)),
            ("version", Value::from(&user.version)),
            ("first_name", Value::from(&user.profile.first_name)),
            ("last_name", Value::from(&user.profile.last_name)),
            ("gender", Value::from(&user.profile.gender)),
            ("date_of_birth", Value::from(&user.profile.date_of_birth)),
            ("social_profiles", Value::from(&user.social_profiles)),
            ("has_client_details", Value::from(user.client_details.is_some())),
            ("client_details", Value::from(&user.client_details)),
            ("api_keys", Value::from(&user.api_keys)),
        ]);

        self.db.execute(transaction, params).await.map_err(|e| {
            self.event_logger.log_event(
                SystemEvent::DatabaseError(e.to_string()),
                Severity::Error,
            );
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    // I'll continue with the rest of the implementation in the next message...
}
impl UserDatabase {
    // ... previous implementation ...

    #[instrument(skip(self))]
    async fn get_user_with_history(&self, user_id: &Uuid7, version: Option<Version>) -> Result<User> {
        let query = r#"
            SELECT 
                *,
                ->has_profile->(
                    SELECT * FROM user_profile 
                    WHERE version <= $version 
                    ORDER BY version.timestamp DESC 
                    LIMIT 1
                ) AS profile,
                ->has_social->(
                    SELECT * FROM social_profile 
                    WHERE version <= $version 
                    ORDER BY version.timestamp DESC
                ) AS social_profiles,
                ->has_details->(
                    SELECT * FROM client_details 
                    WHERE version <= $version 
                    ORDER BY version.timestamp DESC 
                    LIMIT 1
                ) AS client_details,
                ->owns_listing->listing.* AS listings,
                ->has_api_key->(
                    SELECT * FROM api_key 
                    WHERE version <= $version 
                    ORDER BY version.timestamp DESC
                ) AS api_keys
            FROM user 
            WHERE user_id = $user_id 
            AND version <= $version 
            ORDER BY version.timestamp DESC 
            LIMIT 1
        "#;

        let params = Object::from([
            ("user_id", Value::from(user_id)),
            ("version", Value::from(version.unwrap_or_else(|| Version::latest()))),
        ]);

        let user: Option<User> = self.db.query(query)
            .bind(params)
            .await?
            .take(0)?;

        user.ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    #[instrument(skip(self))]
    async fn update_user(&self, user: &User) -> Result<()> {
        let transaction = r#"
            BEGIN TRANSACTION;

            // Update base user with new version
            LET $user = (
                UPDATE user 
                SET {
                    email: $email,
                    secondary_email: $secondary_email,
                    main_phone_number: $main_phone_number,
                    secondary_phone_number: $secondary_phone_number,
                    nationality: $nationality,
                    speaking_languages: $speaking_languages,
                    last_login_at: time::now(),
                    version: $version
                }
                WHERE user_id = $user_id
                RETURN AFTER
            );

            // Update profile with version tracking
            LET $profile = (
                UPDATE user_profile 
                SET {
                    first_name: $first_name,
                    last_name: $last_name,
                    gender: $gender,
                    date_of_birth: $date_of_birth,
                    version: $version
                }
                FROM $user->has_profile->user_profile
                RETURN AFTER
            );

            // Handle social profiles
            DELETE ->has_social->social_profile FROM $user;
            
            FOR $social IN $social_profiles {
                LET $new_social = (
                    CREATE social_profile CONTENT {
                        platform: $social.platform,
                        username: $social.username,
                        created_at: time::now(),
                        version: $version
                    }
                );
                
                RELATE $user->has_social->$new_social CONTENT {
                    created_at: time::now(),
                    version: $version
                };
            };

            // Update client details if present
            IF $has_client_details {
                LET $details = (
                    UPDATE client_details 
                    SET {
                        budget: $client_details.budget,
                        preferences: $client_details.preferences,
                        family_status: $client_details.family_status,
                        income: $client_details.income,
                        property_preferences: $client_details.property_preferences,
                        searching_for: $client_details.searching_for,
                        occupation: $client_details.occupation,
                        price_range: $client_details.price_range,
                        characteristics: $client_details.characteristics,
                        version: $version
                    }
                    FROM $user->has_details->client_details
                    RETURN AFTER
                );
            };

            // Update API keys with version tracking
            DELETE ->has_api_key->api_key FROM $user;
            
            FOR $key IN $api_keys {
                LET $new_key = (
                    CREATE api_key CONTENT {
                        key: $key.key,
                        created_at: time::now(),
                        expires_at: $key.expires_at,
                        version: $version
                    }
                );
                
                RELATE $user->has_api_key->$new_key CONTENT {
                    created_at: time::now(),
                    version: $version
                };
            };

            COMMIT TRANSACTION;
        "#;

        let params = Object::from([
            ("user_id", Value::from(&user.user_id)),
            ("email", Value::from(&user.email)),
            ("secondary_email", Value::from(&user.secondary_email)),
            ("main_phone_number", Value::from(&user.main_phone_number)),
            ("secondary_phone_number", Value::from(&user.secondary_phone_number)),
            ("nationality", Value::from(&user.nationality)),
            ("speaking_languages", Value::from(&user.speaking_languages)),
            ("version", Value::from(&user.version)),
            ("first_name", Value::from(&user.profile.first_name)),
            ("last_name", Value::from(&user.profile.last_name)),
            ("gender", Value::from(&user.profile.gender)),
            ("date_of_birth", Value::from(&user.profile.date_of_birth)),
            ("social_profiles", Value::from(&user.social_profiles)),
            ("has_client_details", Value::from(user.client_details.is_some())),
            ("client_details", Value::from(&user.client_details)),
            ("api_keys", Value::from(&user.api_keys)),
        ]);

        self.db.execute(transaction, params).await.map_err(|e| {
            self.event_logger.log_event(
                SystemEvent::DatabaseError(e.to_string()),
                Severity::Error,
            );
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }


    #[instrument(skip(self))]
    async fn delete_user(&self, user_id: &Uuid7) -> Result<()> {
        let transaction = r#"
            BEGIN TRANSACTION;
            
            // Archive the user and all related data instead of hard delete
            LET $user = SELECT * FROM user WHERE user_id = $user_id;
            
            // Archive all relationships and related entities
            LET $archived = (
                CREATE archived_user CONTENT {
                    user_data: $user,
                    profile: (SELECT * FROM $user->has_profile->user_profile),
                    social_profiles: (SELECT * FROM $user->has_social->social_profile),
                    client_details: (SELECT * FROM $user->has_details->client_details),
                    listings: (SELECT * FROM $user->owns_listing->listing),
                    api_keys: (SELECT * FROM $user->has_api_key->api_key),
                    archived_at: time::now(),
                    version: $version
                }
            );
            
            // Delete relationships first
            DELETE ->has_profile->user_profile FROM $user;
            DELETE ->has_social->social_profile FROM $user;
            DELETE ->has_details->client_details FROM $user;
            DELETE ->owns_listing->listing FROM $user;
            DELETE ->has_api_key->api_key FROM $user;
            
            // Delete the user record
            DELETE user WHERE user_id = $user_id;
            
            COMMIT TRANSACTION;
        "#;

        let params = Object::from([
            ("user_id", Value::from(user_id)),
            ("version", Value::from(&Version::new())),
        ]);

        self.db.execute(transaction, params).await.map_err(|e| {
            self.event_logger.log_event(
                SystemEvent::DatabaseError(e.to_string()),
                Severity::Error,
            );
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn initialize_schema(&self) -> Result<()> {
        let schema = r#"
            -- Base Tables with Version Control
            DEFINE TABLE user SCHEMAFULL
                PERMISSIONS 
                    FOR select, create WHERE $auth.role = "admin" OR id = $auth.id
                    FOR update, delete WHERE $auth.role = "admin";
            
            DEFINE FIELD user_id ON user TYPE string ASSERT $value != NONE;
            DEFINE FIELD email ON user TYPE string ASSERT is::email($value);
            DEFINE FIELD secondary_email ON user TYPE option<string> ASSERT $value = NONE OR is::email($value);
            DEFINE FIELD main_phone_number ON user TYPE string;
            DEFINE FIELD secondary_phone_number ON user TYPE option<string>;
            DEFINE FIELD nationality ON user TYPE string;
            DEFINE FIELD speaking_languages ON user TYPE array<string>;
            DEFINE FIELD created_at ON user TYPE datetime VALUE $value OR time::now();
            DEFINE FIELD last_login_at ON user TYPE datetime;
            DEFINE FIELD version ON user TYPE object {
                major: number,
                minor: number,
                patch: number,
                timestamp: datetime,
                hash: string
            };

            -- Profile Table
            DEFINE TABLE user_profile SCHEMAFULL;
            DEFINE FIELD first_name ON user_profile TYPE string;
            DEFINE FIELD last_name ON user_profile TYPE string;
            DEFINE FIELD gender ON user_profile TYPE string;
            DEFINE FIELD date_of_birth ON user_profile TYPE datetime;
            DEFINE FIELD created_at ON user_profile TYPE datetime;
            DEFINE FIELD version ON user_profile TYPE object;

            -- Social Profile Table
            DEFINE TABLE social_profile SCHEMAFULL;
            DEFINE FIELD platform ON social_profile TYPE string;
            DEFINE FIELD username ON social_profile TYPE string;
            DEFINE FIELD created_at ON social_profile TYPE datetime;
            DEFINE FIELD version ON social_profile TYPE object;

            -- Client Details Table
            DEFINE TABLE client_details SCHEMAFULL;
            DEFINE FIELD budget ON client_details TYPE string;
            DEFINE FIELD preferences ON client_details TYPE array<string>;
            DEFINE FIELD family_status ON client_details TYPE string;
            DEFINE FIELD income ON client_details TYPE float;
            DEFINE FIELD property_preferences ON client_details TYPE array<string>;
            DEFINE FIELD searching_for ON client_details TYPE string;
            DEFINE FIELD occupation ON client_details TYPE string;
            DEFINE FIELD price_range ON client_details TYPE object {
                min: float,
                max: float
            };
            DEFINE FIELD characteristics ON client_details TYPE array<string>;
            DEFINE FIELD created_at ON client_details TYPE datetime;
            DEFINE FIELD version ON client_details TYPE object;

            -- API Key Table
            DEFINE TABLE api_key SCHEMAFULL;
            DEFINE FIELD key ON api_key TYPE string;
            DEFINE FIELD created_at ON api_key TYPE datetime;
            DEFINE FIELD expires_at ON api_key TYPE datetime;
            DEFINE FIELD version ON api_key TYPE object;

            -- Archive Table for Soft Deletes
            DEFINE TABLE archived_user SCHEMAFULL;
            DEFINE FIELD user_data ON archived_user TYPE object;
            DEFINE FIELD profile ON archived_user TYPE object;
            DEFINE FIELD social_profiles ON archived_user TYPE array;
            DEFINE FIELD client_details ON archived_user TYPE option<object>;
            DEFINE FIELD listings ON archived_user TYPE array;
            DEFINE FIELD api_keys ON archived_user TYPE array;
            DEFINE FIELD archived_at ON archived_user TYPE datetime;
            DEFINE FIELD version ON archived_user TYPE object;

            -- Relationships
            DEFINE TABLE has_profile SCHEMAFULL;
            DEFINE FIELD created_at ON has_profile TYPE datetime;
            DEFINE FIELD version ON has_profile TYPE object;

            DEFINE TABLE has_social SCHEMAFULL;
            DEFINE FIELD created_at ON has_social TYPE datetime;
            DEFINE FIELD version ON has_social TYPE object;

            DEFINE TABLE has_details SCHEMAFULL;
            DEFINE FIELD created_at ON has_details TYPE datetime;
            DEFINE FIELD version ON has_details TYPE object;

            DEFINE TABLE owns_listing SCHEMAFULL;
            DEFINE FIELD created_at ON owns_listing TYPE datetime;
            DEFINE FIELD version ON owns_listing TYPE object;

            DEFINE TABLE has_api_key SCHEMAFULL;
            DEFINE FIELD created_at ON has_api_key TYPE datetime;
            DEFINE FIELD version ON has_api_key TYPE object;

            -- Indexes for Performance
            DEFINE INDEX user_email ON user FIELDS email UNIQUE;
            DEFINE INDEX user_phone ON user FIELDS main_phone_number UNIQUE;
            DEFINE INDEX api_key_value ON api_key FIELDS key UNIQUE;

            -- Event Triggers for Version Control
            DEFINE EVENT version_update ON TABLE user WHEN $event = "UPDATE" THEN {
                LET $new_version = {
                    major: $value.version.major,
                    minor: $value.version.minor + 1,
                    patch: 0,
                    timestamp: time::now(),
                    hash: crypto::md5($value)
                };
                UPDATE $this SET version = $new_version;
            };
        "#;

        self.db.execute(schema).await.map_err(|e| {
            self.event_logger.log_event(
                SystemEvent::DatabaseError(e.to_string()),
                Severity::Error,
            );
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn manage_preferences(&self, user_id: &Uuid7, preferences: &[ClientPreferences]) -> Result<()> {
        let transaction = r#"
            BEGIN TRANSACTION;
            
            LET $user = SELECT * FROM user WHERE user_id = $user_id;
            
            // Update existing preferences
            UPDATE client_details 
            SET preferences = $new_preferences
            FROM $user->has_details->client_details;
            
            // Track preference history
            CREATE preference_history CONTENT {
                user_id: $user_id,
                preferences: $preferences,
                timestamp: time::now(),
                version: $version
            };
            
            COMMIT TRANSACTION;
        "#;

        let params = Object::from([
            ("user_id", Value::from(user_id)),
            ("new_preferences", Value::from(preferences)),
            ("version", Value::from(&Version::new())),
        ]);

        self.db.execute(transaction, params).await.map_err(|e| {
            self.event_logger.log_event(
                SystemEvent::DatabaseError(e.to_string()),
                Severity::Error,
            );
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}