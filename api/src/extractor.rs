use async_trait::async_trait;
use axum::{
    extract::{Extension, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, HeaderValue},
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use time::{Duration, OffsetDateTime};

use crate::{ApiContext, Error};

const DEFAULT_SESSION_LENGTH: Duration = Duration::weeks(2);

const SCHEME_PREFIX: &str = "Bearer ";

pub struct AuthUser {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct AuthUserClaims {
    user_id: i32,
    exp: i64,
}

impl AuthUser {
    pub(crate) fn to_jwt(&self, ctx: &ApiContext) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id,
            exp: { OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH }.unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing cannot fail")
    }

    fn from_authorization(ctx: &ApiContext, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::debug!("Authorization header is not valid UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!("Authorization header does not start with {}", SCHEME_PREFIX);
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                log::debug!("JWT parsing failed: {}", e);
                Error::Unauthorized
            })?;

        let hmac = Hmac::<Sha384>::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            log::debug!("JWT verification failed: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("JWT has expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request_parts(parts, state)
            .await
            .expect("ApiContext should be attached to the request");

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}
