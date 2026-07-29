use ai_agent_assistant_lib::credentials::{
    CloudflareAccessCredentialError, CloudflareAccessCredentialItem,
};

#[test]
fn public_errors_are_closed_and_redacted() {
    let cases = [
        (
            CloudflareAccessCredentialError::Missing(CloudflareAccessCredentialItem::ClientId),
            "the Cloudflare Access client ID item is missing",
        ),
        (
            CloudflareAccessCredentialError::AccessDenied(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            "access to the Cloudflare Access client secret item was denied",
        ),
        (
            CloudflareAccessCredentialError::Cancelled(
                CloudflareAccessCredentialItem::ClientSecret,
            ),
            "the Cloudflare Access client secret read was cancelled",
        ),
        (
            CloudflareAccessCredentialError::PlatformFailure,
            "the macOS Keychain credential proof failed",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        let output = format!("{error:?}");
        for prohibited in [
            "io.cortexa.demo.cloudflare-access",
            "fake-client",
            "OSStatus",
            "-25300",
        ] {
            assert!(!output.contains(prohibited));
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[test]
fn probe_fails_closed_off_macos() {
    use ai_agent_assistant_lib::credentials::probe_cloudflare_access_credentials;

    assert_eq!(
        probe_cloudflare_access_credentials(),
        Err(CloudflareAccessCredentialError::UnsupportedPlatform)
    );
}
