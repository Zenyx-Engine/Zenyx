use thiserror::Error;


#[derive(Debug,Error)]  
enum ZError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error)

}