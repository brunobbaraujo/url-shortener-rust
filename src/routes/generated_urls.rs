use axum::extract::Path;
async fn path(Path(shortened_url): Path<u32>) {}
