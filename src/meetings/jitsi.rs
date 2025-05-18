use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

use crate::models::{
    User,
    meeting::{Meeting, MeetingDTO},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiClaims {
    pub aud: String,
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
    pub nbf: u64,
    pub sub: String,
    pub context: JitsiContext,
    pub room: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiContext {
    pub features: JitsiFeatures,
    pub user: JitsiUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiFeatures {
    pub livestreaming: bool,
    #[serde(rename = "outbound-call")]
    pub outbound_call: bool,
    #[serde(rename = "sip-outbound-call")]
    pub sip_outbound_call: bool,
    pub transcription: bool,
    pub recording: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JitsiUser {
    #[serde(rename = "hidden-from-recorder")]
    pub hidden_from_recorder: bool,
    pub moderator: bool,
    pub name: String,
    pub id: String,
    pub avatar: String,
    pub email: String,
}

pub fn build_jitsi_session(
    meeting: &Meeting,
    user: &User,
    app_id: &str,
    secret: &str,
    kid: &str,
) -> MeetingDTO {
    let mut claims = JitsiClaims {
        aud: "jitsi".into(),
        iss: "chat".into(),
        iat: meeting.starts_at,
        exp: meeting.expires_at,
        nbf: meeting.starts_at,
        sub: app_id.into(),
        context: JitsiContext {
            features: JitsiFeatures {
                livestreaming: true,
                outbound_call: true,
                sip_outbound_call: false,
                transcription: true,
                recording: true,
            },
            user: JitsiUser {
                hidden_from_recorder: false,
                moderator: false,
                name: user.fullname().clone(),
                id: user.user_id().to_string(),
                avatar: "".into(),
                email: user.username().clone(),
            },
        },
        room: meeting.id.clone(),
    };

    if user.is_student() {
        claims.context.user.moderator = false;
    } else {
        claims.context.user.moderator = true;
    }

    let header = jsonwebtoken::Header {
        alg: jsonwebtoken::Algorithm::RS256,
        kid: Some(kid.into()),
        ..Default::default()
    };

    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(secret.as_bytes()).unwrap(),
    )
    .unwrap();

    MeetingDTO {
        meeting_id: meeting.id.clone(),
        join_url: format!("https://8x8.vc/{}/{}?jwt={token}", app_id, claims.room),
        expires_at: "".into(),
        starts_at: "".into(),
        auth_token: Some(token),
    }
}
