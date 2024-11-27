use domain::entities::stub_domain_entity::StubEntity;
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelBehavior, ActiveValue, DeriveEntityModel,
    DerivePrimaryKey, EntityTrait, EnumIter, FromJsonQueryResult, PrimaryKeyTrait, Related,
    RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stub_entity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub value: KeyValue,
    pub auto_ref: Option<i32>,
}

impl Model {
    pub fn to_domain(&self) -> StubEntity {
        StubEntity {
            id: Some(self.id),
            name: self.name.clone(),
            value: domain::entities::stub_domain_entity::KeyValue {
                id: self.value.id,
                name: self.value.name.clone(),
            },
            auto_ref: self.auto_ref,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AutoRef,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AutoRef => Entity::has_many(super::stub_database_entity::Entity).into(),
        }
    }
}

impl Related<super::stub_database_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AutoRef.def()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct KeyValue {
    pub id: i32,
    pub name: String,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn from_domain(entity: &StubEntity, set_id: bool) -> Self {
        ActiveModel {
            id: if set_id {
                ActiveValue::Set(entity.id.unwrap_or_default())
            } else {
                ActiveValue::NotSet
            },
            name: ActiveValue::Set(entity.name.clone()),
            value: ActiveValue::Set(KeyValue {
                id: entity.value.id,
                name: entity.value.name.clone(),
            }),
            auto_ref: ActiveValue::Set(entity.auto_ref),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_domain() {
        let model = Model {
            id: 1,
            name: "Test".to_string(),
            value: KeyValue { id: 1, name: "Value".to_string() },
            auto_ref: Some(2),
        };

        let domain_entity = model.to_domain();

        assert_eq!(domain_entity.id, Some(1));
        assert_eq!(domain_entity.name, "Test");
        assert_eq!(domain_entity.value.id, 1);
        assert_eq!(domain_entity.value.name, "Value");
        assert_eq!(domain_entity.auto_ref, Some(2));
    }

    #[test]
    fn test_active_model_from_domain_with_id() {
        let domain_entity = StubEntity {
            id: Some(1),
            name: "Test".to_string(),
            value: domain::entities::stub_domain_entity::KeyValue { id: 1, name: "Value".to_string() },
            auto_ref: Some(2),
        };

        let active_model = ActiveModel::from_domain(&domain_entity, true);

        assert_eq!(active_model.id, ActiveValue::Set(1));
        assert_eq!(active_model.name, ActiveValue::Set("Test".to_string()));
        assert_eq!(active_model.value, ActiveValue::Set(KeyValue { id: 1, name: "Value".to_string() }));
        assert_eq!(active_model.auto_ref, ActiveValue::Set(Some(2)));
    }

    #[test]
    fn test_active_model_from_domain_without_id() {
        let domain_entity = StubEntity {
            id: Some(1),
            name: "Test".to_string(),
            value: domain::entities::stub_domain_entity::KeyValue { id: 1, name: "Value".to_string() },
            auto_ref: Some(2),
        };

        let active_model = ActiveModel::from_domain(&domain_entity, false);

        assert_eq!(active_model.id, ActiveValue::NotSet);
        assert_eq!(active_model.name, ActiveValue::Set("Test".to_string()));
        assert_eq!(active_model.value, ActiveValue::Set(KeyValue { id: 1, name: "Value".to_string() }));
        assert_eq!(active_model.auto_ref, ActiveValue::Set(Some(2)));
    }
}
