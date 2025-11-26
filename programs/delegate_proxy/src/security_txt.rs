#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "SkyPay",
    project_url: "https://sky-pay.dev",
    contacts: "email:security@sky-pay.dev",
    policy: "https://sky-pay.dev/disclosure-policy.html",

    // Optional Fields
    preferred_languages: "en"
}