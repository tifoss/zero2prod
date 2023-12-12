use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().into()
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
