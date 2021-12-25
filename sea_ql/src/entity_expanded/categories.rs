//! SeaORM Entity. Generated by sea-orm-codegen 0.4.2

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "categories"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub uuid: String,
    pub name: String,
    pub sub_category_uuid: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Uuid,
    Name,
    SubCategoryUuid,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Uuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SubCategories,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Uuid => ColumnType::String(Some(50u32)).def(),
            Self::Name => ColumnType::Text.def(),
            Self::SubCategoryUuid => ColumnType::Text.def().null(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def().null(),
            Self::DeletedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::SubCategories => Entity::belongs_to(super::sub_categories::Entity)
                .from(Column::SubCategoryUuid)
                .to(super::sub_categories::Column::Uuid)
                .into(),
        }
    }
}

impl Related<super::sub_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
