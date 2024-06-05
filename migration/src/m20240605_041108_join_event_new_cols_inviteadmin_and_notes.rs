use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(JoinEvent::Table)
                    .add_column(ColumnDef::new(JoinEvent::InviteAdmin).not_null().integer())
                    .add_column(ColumnDef::new(JoinEvent::Notes).text().not_null())
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("FK_invite_admin")
                            .from_tbl(JoinEvent::Table)
                            .from_col(JoinEvent::InviteAdmin)
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(JoinEvent::Table)
                    .drop_column(JoinEvent::InviteAdmin)
                    .drop_column(JoinEvent::Notes)
                    .drop_foreign_key(Alias::new("FK_invite_admin"))
                    .take(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum JoinEvent {
    Table,
    InviteAdmin,
    Notes,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
