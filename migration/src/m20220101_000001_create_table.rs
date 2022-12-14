use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use entity::{author, post, post_media, telegram_user};

        manager
            .create_table(
                sea_query::Table::create()
                    .table(telegram_user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(telegram_user::Column::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(telegram_user::Column::Channel)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(telegram_user::Column::PowerLevel)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(telegram_user::Column::LastFeedId)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                sea_query::Table::create()
                    .table(author::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(author::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(author::Column::PlatformId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(author::Column::Name).string().not_null())
                    .col(ColumnDef::new(author::Column::Username).string().not_null())
                    .col(ColumnDef::new(author::Column::AvatarUrl).string().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                sea_query::Table::create()
                    .table(post::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(post::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(post::Column::PlatformId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(post::Column::AuthorId).integer().not_null())
                    .col(ColumnDef::new(post::Column::Text).string().not_null())
                    .col(ColumnDef::new(post::Column::SourceUrl).string().not_null())
                    .col(ColumnDef::new(post::Column::SourceText).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                sea_query::Table::create()
                    .table(post_media::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(post_media::Column::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(post_media::Column::PostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(post_media::Column::MediaType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(post_media::Column::MediaUrl)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::telegram_user::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::author::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::post::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::post_media::Entity)
                    .to_owned(),
            )
            .await
    }
}
