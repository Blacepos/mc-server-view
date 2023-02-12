

#[get("/")]
pub fn index() -> &'static str {
    "Yo what's up. The frontend isn't started yet and the backend is completely untested. Use the API endpoints: /api/start, /api/query, /api/address."
}
