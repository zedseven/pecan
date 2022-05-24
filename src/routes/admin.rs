// Uses
use rocket::{routes, Route};

use super::Routable;

/// The route for this section.
pub(super) struct AdminApi;
impl Routable for AdminApi {
	const PATH: &'static str = "/admin";
	const ROUTES: &'static dyn Fn() -> Vec<Route> = &|| routes![];
}
