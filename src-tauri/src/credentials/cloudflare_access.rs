use std::fmt;

use thiserror::Error;

const SERVICE_LABEL: &str = "io.cortexa.demo.cloudflare-access";
const CLIENT_ID_ACCOUNT: &str = "client-id";
const CLIENT_SECRET_ACCOUNT: &str = "client-secret";
const MAX_CREDENTIAL_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloudflareAccessCredentialStatus {
    Available,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloudflareAccessCredentialItem {
    ClientId,
    ClientSecret,
}

impl fmt::Display for CloudflareAccessCredentialItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientId => formatter.write_str("client ID"),
            Self::ClientSecret => formatter.write_str("client secret"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CloudflareAccessCredentialError {
    #[error("the Cloudflare Access {0} item is missing")]
    Missing(CloudflareAccessCredentialItem),
    #[error("access to the Cloudflare Access {0} item was denied")]
    AccessDenied(CloudflareAccessCredentialItem),
    #[error("the Cloudflare Access {0} read was cancelled")]
    Cancelled(CloudflareAccessCredentialItem),
    #[error("the Cloudflare Access {0} item has an invalid value")]
    InvalidValue(CloudflareAccessCredentialItem),
    #[error("the macOS Keychain credential proof is unavailable on this platform")]
    UnsupportedPlatform,
    #[error("the macOS Keychain credential proof failed")]
    PlatformFailure,
}

pub fn probe_cloudflare_access_credentials(
) -> Result<CloudflareAccessCredentialStatus, CloudflareAccessCredentialError> {
    #[cfg(target_os = "macos")]
    {
        probe_with_reader(&MacOsKeychainReader)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(CloudflareAccessCredentialError::UnsupportedPlatform)
    }
}

trait ReadOnlyCredentialSource {
    fn read(
        &self,
        service: &'static str,
        account: &'static str,
        item: CloudflareAccessCredentialItem,
    ) -> Result<FakeCredentialBytes, CloudflareAccessCredentialError>;
}

fn probe_with_reader(
    source: &impl ReadOnlyCredentialSource,
) -> Result<CloudflareAccessCredentialStatus, CloudflareAccessCredentialError> {
    let client_id = source.read(
        SERVICE_LABEL,
        CLIENT_ID_ACCOUNT,
        CloudflareAccessCredentialItem::ClientId,
    )?;
    validate_secret(&client_id, CloudflareAccessCredentialItem::ClientId)?;

    let client_secret = source.read(
        SERVICE_LABEL,
        CLIENT_SECRET_ACCOUNT,
        CloudflareAccessCredentialItem::ClientSecret,
    )?;
    validate_secret(&client_secret, CloudflareAccessCredentialItem::ClientSecret)?;

    Ok(CloudflareAccessCredentialStatus::Available)
}

fn validate_secret(
    value: &FakeCredentialBytes,
    item: CloudflareAccessCredentialItem,
) -> Result<(), CloudflareAccessCredentialError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > MAX_CREDENTIAL_BYTES
        || !bytes.iter().all(u8::is_ascii_graphic)
    {
        return Err(CloudflareAccessCredentialError::InvalidValue(item));
    }

    Ok(())
}

struct FakeCredentialBytes(Vec<u8>);

impl FakeCredentialBytes {
    fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for FakeCredentialBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

#[cfg(target_os = "macos")]
struct MacOsKeychainReader;

#[cfg(target_os = "macos")]
impl ReadOnlyCredentialSource for MacOsKeychainReader {
    fn read(
        &self,
        service: &'static str,
        account: &'static str,
        item: CloudflareAccessCredentialItem,
    ) -> Result<FakeCredentialBytes, CloudflareAccessCredentialError> {
        use security_framework::passwords::get_generic_password;
        use security_framework_sys::base::{
            errSecAuthFailed as ERR_SEC_AUTH_FAILED, errSecItemNotFound as ERR_SEC_ITEM_NOT_FOUND,
        };

        const ERR_SEC_USER_CANCELLED: i32 = -128;
        const ERR_SEC_INTERACTION_NOT_ALLOWED: i32 = -25_308;

        get_generic_password(service, account)
            .map(FakeCredentialBytes::new)
            .map_err(|error| match error.code() {
                ERR_SEC_ITEM_NOT_FOUND => CloudflareAccessCredentialError::Missing(item),
                ERR_SEC_AUTH_FAILED | ERR_SEC_INTERACTION_NOT_ALLOWED => {
                    CloudflareAccessCredentialError::AccessDenied(item)
                }
                ERR_SEC_USER_CANCELLED => CloudflareAccessCredentialError::Cancelled(item),
                _ => CloudflareAccessCredentialError::PlatformFailure,
            })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    const FAKE_CLIENT_ID: &[u8] = b"fake-client-id.access";
    const FAKE_CLIENT_SECRET: &[u8] = b"fake-client-secret";

    struct FakeSource {
        calls: RefCell<Vec<(&'static str, &'static str, CloudflareAccessCredentialItem)>>,
        client_id: Result<Vec<u8>, CloudflareAccessCredentialError>,
        client_secret: Result<Vec<u8>, CloudflareAccessCredentialError>,
    }

    impl FakeSource {
        fn available() -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                client_id: Ok(FAKE_CLIENT_ID.to_vec()),
                client_secret: Ok(FAKE_CLIENT_SECRET.to_vec()),
            }
        }
    }

    impl ReadOnlyCredentialSource for FakeSource {
        fn read(
            &self,
            service: &'static str,
            account: &'static str,
            item: CloudflareAccessCredentialItem,
        ) -> Result<FakeCredentialBytes, CloudflareAccessCredentialError> {
            self.calls.borrow_mut().push((service, account, item));
            let result = match item {
                CloudflareAccessCredentialItem::ClientId => &self.client_id,
                CloudflareAccessCredentialItem::ClientSecret => &self.client_secret,
            };

            result
                .as_ref()
                .map(|bytes| FakeCredentialBytes::new(bytes.clone()))
                .map_err(|error| *error)
        }
    }

    #[test]
    fn reads_only_the_two_fixed_labels_in_order() {
        let source = FakeSource::available();

        let result = probe_with_reader(&source);

        assert_eq!(result, Ok(CloudflareAccessCredentialStatus::Available));
        assert_eq!(
            source.calls.into_inner(),
            vec![
                (
                    SERVICE_LABEL,
                    CLIENT_ID_ACCOUNT,
                    CloudflareAccessCredentialItem::ClientId
                ),
                (
                    SERVICE_LABEL,
                    CLIENT_SECRET_ACCOUNT,
                    CloudflareAccessCredentialItem::ClientSecret
                ),
            ]
        );
    }

    #[test]
    fn fails_closed_when_the_client_id_is_missing() {
        let source = FakeSource {
            client_id: Err(CloudflareAccessCredentialError::Missing(
                CloudflareAccessCredentialItem::ClientId,
            )),
            ..FakeSource::available()
        };

        let result = probe_with_reader(&source);

        assert_eq!(
            result,
            Err(CloudflareAccessCredentialError::Missing(
                CloudflareAccessCredentialItem::ClientId
            ))
        );
        assert_eq!(source.calls.borrow().len(), 1);
    }

    #[test]
    fn preserves_closed_denial_and_cancellation_outcomes() {
        for error in [
            CloudflareAccessCredentialError::AccessDenied(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            CloudflareAccessCredentialError::Cancelled(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
        ] {
            let source = FakeSource {
                client_secret: Err(error),
                ..FakeSource::available()
            };

            assert_eq!(probe_with_reader(&source), Err(error));
        }
    }

    #[test]
    fn rejects_empty_oversized_and_control_character_values() {
        for bytes in [
            Vec::new(),
            vec![b'x'; MAX_CREDENTIAL_BYTES + 1],
            b"fake\nsecret".to_vec(),
        ] {
            let source = FakeSource {
                client_secret: Ok(bytes),
                ..FakeSource::available()
            };

            assert_eq!(
                probe_with_reader(&source),
                Err(CloudflareAccessCredentialError::InvalidValue(
                    CloudflareAccessCredentialItem::ClientSecret
                ))
            );
        }
    }

    #[test]
    fn error_output_contains_no_values_or_native_details() {
        for error in [
            CloudflareAccessCredentialError::Missing(CloudflareAccessCredentialItem::ClientId),
            CloudflareAccessCredentialError::AccessDenied(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            CloudflareAccessCredentialError::Cancelled(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            CloudflareAccessCredentialError::InvalidValue(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            CloudflareAccessCredentialError::PlatformFailure,
        ] {
            let output = format!("{error} {error:?}");
            for prohibited in [
                "fake-client-id.access",
                "fake-client-secret",
                "io.cortexa.demo.cloudflare-access",
                "-25300",
                "OSStatus",
            ] {
                assert!(!output.contains(prohibited));
            }
        }
    }
}
