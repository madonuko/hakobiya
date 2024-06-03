//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "join_sub_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user: i32,
    pub sub_event: i32,
    pub scanned: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sub_event::Entity",
        from = "Column::SubEvent",
        to = "super::sub_event::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SubEvent,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::sub_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubEvent.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}