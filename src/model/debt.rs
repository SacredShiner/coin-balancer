//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "debt")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub transaction_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub debtor_id: String,
    pub amount: i32,
    pub was_split_unequally: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::DebtorId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::transaction::Entity",
        from = "Column::TransactionId",
        to = "super::transaction::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Transaction,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
