use patch_ts::remote::fetch_http;

#[test]
fn test_fetch_http_not_found() {
    // Attempt to fetch from a non‑existent URL – should error
    let result = fetch_http("http://localhost:59999/doesnotexist");
    assert!(result.is_err());
}

// For Git, we skip automated test because it requires a real repository.
// Manual testing will be done.
