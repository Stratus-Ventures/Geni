use std::sync::Arc;
use async_trait::async_trait;
use webauthn_rs::prelude::*;
use webauthn_rs::Webauthn;
use url::Url;
use uuid::Uuid;
use crate::domain::ports::WebAuthnVerifier;
use crate::domain::errors::WebAuthnError;
use crate::infra::config::AppConfig;
use crate::domain::models::Passkey; // Import domain Passkey
use base64::Engine;

pub struct WebAuthnAdapter {
    webauthn: Arc<Webauthn>,
}

impl WebAuthnAdapter {
    pub fn new(config: &AppConfig) -> Self {
        let rp_id = &config.rp_id;
        let rp_origin = Url::parse(&config.rp_origin).expect("Invalid RP Origin URL");
        let builder = WebauthnBuilder::new(rp_id, &rp_origin).expect("Invalid WebAuthn Configuration");
        let webauthn = builder.rp_name(&config.rp_name).build().expect("Failed to build WebAuthn instance");

        Self {
            webauthn: Arc::new(webauthn),
        }
    }
}

#[async_trait]
impl WebAuthnVerifier for WebAuthnAdapter {
    async fn start_registration(
        &self,
        user_id: Uuid,
        email: &str,
        name: &str,
    ) -> Result<(String, String), WebAuthnError> {
        let (ccr, state) = self.webauthn
            .start_passkey_registration(user_id, email, name, None)
            .map_err(|e| WebAuthnError::Internal(format!("Start reg failed: {}", e)))?;

        let ccr_json = serde_json::to_string(&ccr)
            .map_err(|e| WebAuthnError::Internal(format!("CCR Serialize failed: {}", e)))?;
        let state_json = serde_json::to_string(&state)
            .map_err(|e| WebAuthnError::Internal(format!("State Serialize failed: {}", e)))?;

        Ok((ccr_json, state_json))
    }

    async fn verify_registration(
        &self,
        state_json: &str,
        response_json: &str,
    ) -> Result<(String, Vec<u8>), WebAuthnError> {
        let state: PasskeyRegistration = serde_json::from_str(state_json)
            .map_err(|e| WebAuthnError::InvalidData(format!("Invalid state JSON: {}", e)))?;
        
        let response: RegisterPublicKeyCredential = serde_json::from_str(response_json)
            .map_err(|e| WebAuthnError::InvalidData(format!("Invalid response JSON: {}", e)))?;

        let passkey = self.webauthn
            .finish_passkey_registration(&response, &state)
            .map_err(|e| WebAuthnError::VerificationFailed(format!("Verification failed: {}", e)))?;

        let cred_id_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(passkey.cred_id());
        // Use serialized passkey as the public key storage since we can't access fields directly
        let public_key = serde_json::to_vec(&passkey)
            .map_err(|e| WebAuthnError::Internal(format!("Failed to serialize passkey: {}", e)))?;

        Ok((cred_id_b64, public_key))
    }

    async fn start_authentication(
        &self,
        _allow_credentials: &[Passkey],
    ) -> Result<(String, String), WebAuthnError> {
        // Using empty list for now:
        let (rcr, state) = self.webauthn
            .start_passkey_authentication(&[]) 
            .map_err(|e| WebAuthnError::Internal(format!("Start auth failed: {}", e)))?;

        let rcr_json = serde_json::to_string(&rcr)
            .map_err(|e| WebAuthnError::Internal(format!("RCR Serialize failed: {}", e)))?;
        let state_json = serde_json::to_string(&state)
            .map_err(|e| WebAuthnError::Internal(format!("State Serialize failed: {}", e)))?;

        Ok((rcr_json, state_json))
    }

    async fn verify_authentication(
        &self,
        state_json: &str,
        response_json: &str,
    ) -> Result<i64, WebAuthnError> {
        let state: PasskeyAuthentication = serde_json::from_str(state_json)
            .map_err(|e| WebAuthnError::InvalidData(format!("Invalid state JSON: {}", e)))?;
            
        let response: PublicKeyCredential = serde_json::from_str(response_json)
            .map_err(|e| WebAuthnError::InvalidData(format!("Invalid response JSON: {}", e)))?;

        let auth_result = self.webauthn
            .finish_passkey_authentication(&response, &state)
            .map_err(|e| WebAuthnError::VerificationFailed(format!("Auth verification failed: {}", e)))?;
            
        Ok(auth_result.counter() as i64)
    }
}
