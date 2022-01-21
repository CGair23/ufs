use anyhow::Result;
use hyper::{Request, Response, Body, Method, StatusCode};
use routerify_multipart::RequestMultipartExt;   // Import `RequestMultipartExt` trait.
use bytes::Bytes;
use uuid::Uuid;
use tokio::{fs::File, io::AsyncWriteExt};
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

/// Handle incoming request to upload file.
/// This method returns future.
pub async fn upload_service<P: AsRef<Path> + ToString + Display>(
    req: Request<Body>,
    fs_root_dir: Arc<P>
) -> Result<Response<Body>> {
    // Reject requests for non HEAD or GET methods
    if !(req.method() == Method::HEAD || req.method() == Method::POST) {
        let response = Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from(format!("Method not allowed")))?;
        return Ok(response); 
    }
    log::debug!("Convert the request into a `Multipart` instance.");
    // Convert the request into a `Multipart` instance.
    let mut multipart = match req.into_multipart() {
        Ok(m) => m,
        Err(e) => {
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Bad Request: {}", e)))?;
            return Ok(response);
        }
    };

    // handle_multipart(&mut multipart, fs_root_dir).await

    // Handles a single field in a multipart form.
    log::debug!("Handles a single field in a multipart form.");
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?;
    if let Some(mut field) = (&mut multipart).next_field().await? {
        // Get field name.
        let name = field.name();
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();
        log::debug!("Name: {:?}, File Name: {:?}", name, file_name);

        // Process the field data chunks e.g. store them in a file.
        if let Some(chunk) = field.chunk().await? {
            // Do something with field chunk.
            log::debug!("Chunk: {:?}", chunk);
            save_file(chunk, fs_root_dir).await?;
        }else {
            *response.status_mut() = StatusCode::BAD_REQUEST;
            // TODO: update body
        }
    }else {
        *response.status_mut() = StatusCode::BAD_REQUEST;
        // TODO: update body
    }
    Ok(response)

}

/// Saves file data from , optionally overwriting
/// existing file.
///
/// Returns total bytes written to file.
async fn save_file<P: AsRef<Path> + ToString + Display>(
    chunk: Bytes,
    fs_root_dir: Arc<P>
    // overwrite_files: bool,
) -> Result<()> {
    // if !overwrite_files && file_path.exists() { DuplicateFileError }
    let file_uuid = Uuid::new_v4();
    let fs_root_dir_string = fs_root_dir.to_string();
    let file_path = format!("{}/{}.txt", fs_root_dir_string, file_uuid.to_string());
    let mut file = File::create(file_path).await?;
    file.write_all(&chunk).await?;
    Ok(())
}

// Handles a single field in a multipart form
// async fn handle_multipart<'a>(
//     multipart: &'a mut Multipart,
//     fs_root_dir: PathBuf
// ) -> Result<Response<Body>> {
//     let mut response = Response::builder()
//         .status(StatusCode::OK)
//         .body(Body::empty())?;
//     if let Some(mut field) = multipart.next_field().await? {
//         // Get field name.
//         let name = field.name();
//         // Get the field's filename if provided in "Content-Disposition" header.
//         let file_name = field.file_name();
//         log::debug!("Name: {:?}, File Name: {:?}", name, file_name);

//         // Process the field data chunks e.g. store them in a file.
//         if let Some(chunk) = field.chunk().await? {
//             // Do something with field chunk.
//             log::debug!("Chunk: {:?}", chunk);
//             save_file(chunk, fs_root_dir);
//         }else {
//             *response.status_mut() = StatusCode::BAD_REQUEST;
//             // TODO: update body
//         }
//     }else {
//         *response.status_mut() = StatusCode::BAD_REQUEST;
//         // TODO: update body
//     }
//     Ok(response)
// }