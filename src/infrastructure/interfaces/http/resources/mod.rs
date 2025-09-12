use serde::Serialize;

pub mod product_resource;
pub mod user_resource;

#[derive(Serialize)]
pub struct DataResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> DataResponse<T> {
    pub fn success(data: T) -> Self {
        DataResponse {
            success: true,
            data,
        }
    }

    pub fn error(data: T) -> Self {
        DataResponse {
            success: false,
            data,
        }
    }
}
