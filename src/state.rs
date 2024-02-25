use std::sync::Arc;

use crate::prisma::PrismaClient;

use crate::img_user::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

impl AppState {
    pub async fn new(client: Arc<PrismaClient>) -> Self {
        Self {
            user_service: UserService::init(&client),
        }
    }
}
