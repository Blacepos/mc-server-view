

#[get("/")]
pub fn index() -> &'static str {
    "Yo what's up. The frontend isn't finished yet. Use the API endpoints: /api/start, /api/query, /api/address."
}
