use crate::state::AppState;

use utoipa_axum::router::OpenApiRouter;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new().with_state(state)
}
