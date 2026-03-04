#[cfg(test)]
mod tests {
    use diskcortex_daemon::health::HealthResponse;

    #[test]
    fn health_response_has_required_fields() {
        let response = HealthResponse {
            status: "ok".to_string(),
            time: "2024-01-01T00:00:00Z".to_string(),
            version: "0.1.0".to_string(),
        };

        assert_eq!(response.status, "ok");
        assert_eq!(response.version, "0.1.0");
    }
}
