//! Minimal HTTP helper shared by the API client and the geolocation lookup.

use serde::de::DeserializeOwned;

/// GET `url` and decode the JSON body, failing on transport errors, non-2xx
/// statuses, and malformed bodies. Errors are human-readable and include the URL.
pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("Could not reach {url}: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("Request to {url} failed ({status})"));
    }

    resp.json()
        .await
        .map_err(|e| format!("Bad response from {url}: {e}"))
}
