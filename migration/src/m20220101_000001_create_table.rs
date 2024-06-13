use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(User::Mail)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(SubEvent::Table)
                    .col(
                        ColumnDef::new(SubEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SubEvent::Event).integer().not_null())
                    .col(
                        ColumnDef::new(SubEvent::Comment)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SubEvent::Table, SubEvent::Event)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(JoinEvent::Table)
                    .col(
                        ColumnDef::new(JoinEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JoinEvent::User).integer().not_null())
                    .col(ColumnDef::new(JoinEvent::Event).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(JoinEvent::Table, JoinEvent::User)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(JoinEvent::Table, JoinEvent::Event)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(JoinSubEvent::Table)
                    .col(
                        ColumnDef::new(JoinSubEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JoinSubEvent::User).integer().not_null())
                    .col(ColumnDef::new(JoinSubEvent::SubEvent).integer().not_null())
                    .col(
                        ColumnDef::new(JoinSubEvent::Scanned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(JoinSubEvent::Table, JoinSubEvent::User)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(JoinSubEvent::Table, JoinSubEvent::SubEvent)
                            .to(SubEvent::Table, SubEvent::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(HoldEvent::Table)
                    .col(
                        ColumnDef::new(HoldEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HoldEvent::Event).integer().not_null())
                    .col(ColumnDef::new(HoldEvent::Admin).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(HoldEvent::Table, HoldEvent::Event)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(HoldEvent::Table, HoldEvent::Admin)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HoldEvent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(JoinSubEvent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(JoinEvent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SubEvent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Mail,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum SubEvent {
    Table,
    Id,
    Event,
    Comment,
}

#[derive(DeriveIden)]
enum JoinEvent {
    Table,
    Id,
    User,
    Event,
}

#[derive(DeriveIden)]
enum JoinSubEvent {
    Table,
    Id,
    User,
    SubEvent,
    Scanned,
}

#[derive(DeriveIden)]
enum HoldEvent {
    Table,
    Id,
    Event,
    Admin,
}
