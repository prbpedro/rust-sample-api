use sea_orm_migration::prelude::*;


pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241126_000001_create_stub_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StubEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StubEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StubEntity::Name).string().not_null())
                    .col(ColumnDef::new(StubEntity::Value).json_binary())
                    .col(
                        ColumnDef::new(StubEntity::AutoRef)
                        .integer()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-stub-table-ref")
                            .to(StubEntity::Table, StubEntity::Id)
                            .from(StubEntity::Table, StubEntity::AutoRef),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StubEntity::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum StubEntity {
    Table,
    Id,
    Name,
    Value,
    AutoRef
}
