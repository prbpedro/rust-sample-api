use domain::entities::stub_domain_entity::{KeyValue, StubEntity};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct StubEntityAddDto {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(nested)]
    pub value: KeyValueDto,

    #[validate(range(min = 1, message = "auto_ref must be greater than 0"))]
    pub auto_ref: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct KeyValueDto {

    #[validate(range(min = 1, message = "ID must be greater than 0"))]
    pub id: i32,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
}

impl StubEntityAddDto {
    pub fn to_domain(&self) -> StubEntity {
        StubEntity {
            id: None,
            name: self.name.clone(),
            value: self.value.to_domain(),
            auto_ref: self.auto_ref,
        }
    }
}

impl KeyValueDto {
    pub fn to_domain(&self) -> KeyValue {
        KeyValue {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

/// use this to encapsulate fields that require validation
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);



#[derive(Debug, Deserialize, Validate)]
pub struct StubEntityUpdateDto {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,

    #[validate(nested)]
    pub value: Option<KeyValueDto>,

    #[validate(range(min = 1, message = "auto_ref must be greater than 0"))]
    pub auto_ref: Option<i32>,
}
