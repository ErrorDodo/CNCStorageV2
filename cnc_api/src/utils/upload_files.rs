use azure_core::error::Error;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

pub async fn upload_to_azure(
    file_data: &[u8],
    file_name: String,
    file_type: &String,
    user: String,
) -> Result<String, Error> {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("Missing Azure Storage Account");
    let access_key =
        std::env::var("AZURE_STORAGE_ACCESS_KEY").expect("Missing Azure Storage Access Key");

    let container_name = user.to_lowercase();
    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client = BlobServiceClient::new(account.clone(), storage_credentials)
        .container_client(container_name.clone());

    // We use public access here so that we can access the images from the frontend
    // We don't need it on the container level as the backend can access the containers regardless
    container_client
        .create()
        .public_access(PublicAccess::Blob)
        .await?;

    let blob_name = format!("{}", file_name);
    let data_owned = file_data.to_owned();

    container_client
        .blob_client(blob_name.clone())
        .put_block_blob(data_owned)
        .content_type(file_type)
        .await?;

    Ok(format!(
        "https://{}.blob.core.windows.net/{}/{}",
        account, container_name, blob_name
    ))
}
