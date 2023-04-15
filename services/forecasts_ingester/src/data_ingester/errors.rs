use thiserror::Error;

#[derive(Error, Debug)]
pub enum IngestError {
    #[error("database error err={0}")]
    DatabaseError(#[from] Box<dyn sqlx::error::DatabaseError>),

    #[error("persisting data failed err={0}")]
    OtherSqlxError(sqlx::error::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::error::Error> for IngestError {
    fn from(err: sqlx::error::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => IngestError::DatabaseError(db_err),
            other_err => IngestError::OtherSqlxError(other_err),
        }
    }
}
