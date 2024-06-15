//use ntex::http::StatusCode;
use ntex::web::{get, Error as WebError, HttpRequest, HttpResponse};
use ntex_files::NamedFile;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
//use tokio::fs::read;

pub async fn not_found() -> Result<HttpResponse, WebError> {
    let mut content = String::new();

    let not_found_path = Path::new("./html").join("not-found.html");

    if not_found_path.is_file() {
        let mut not_found_file = File::open(not_found_path)?;
        not_found_file.read_to_string(&mut content)?;
        return Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(content));
    }

    return Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body("<h1> 404 Not Found <h1>"));
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let index_path = Path::new("./html").join("index.html");

    if !index_path.is_file() {
        return not_found().await;
    }

    let mut file = File::open(index_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

#[get("/blog")]
pub async fn blog() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let blog_path = Path::new("./html").join("under-construction.html");

    if !blog_path.is_file() {
        return not_found().await;
    }

    let mut file = File::open(blog_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

#[get("/projects")]
pub async fn projects() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let projects_path = Path::new("./html").join("projects.html");

    if !projects_path.is_file() {
        return not_found().await;
    }

    let mut file = File::open(projects_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

/*
#[get("/cv/eng")]
pub async fn cv_english() -> Result<HttpResponse, WebError> {
    let cv_path = Path::new("./assets")
        .join("cvs")
        .join("Oscar_Sjodin_Jansson_-_CV_-_English.pdf");

    if !cv_path.is_file() {
        return not_found().await;
    }

    let file_contents = read(cv_path).await?;
    let response = HttpResponse::build(StatusCode::OK)
        .header("Content-Type", "application/pdf") // Set the appropriate MIME type
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\"",
                "Oscar Sjödin Jansson - CV - English.pdf"
            ),
        ) // Customize the filename as needed
        .body(file_contents);
    Ok(response)
}


#[get("/cv/sv")]
pub async fn cv_swedish() -> Result<HttpResponse, WebError> {
    let cv_path = Path::new("./assets")
        .join("cvs")
        .join("Oscar_Sjodin_Jansson_-_CV_-_Swedish.pdf");

    if !cv_path.is_file() {
        return not_found().await;
    }

    let file_contents = read(cv_path).await?;
    let response = HttpResponse::build(StatusCode::OK)
        .header("Content-Type", "application/pdf") // Set the appropriate MIME type
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\"",
                "Oscar Sjödin Jansson - CV - Swedish.pdf"
            ),
        ) // Customize the filename as needed
        .body(file_contents);
    Ok(response)
}
*/

pub async fn files(req: HttpRequest) -> Result<HttpResponse, WebError> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    let file = NamedFile::open(PathBuf::from("./").join(path));
    if file.is_ok() {
        return Ok(file?.into_response(&req));
    }
    not_found().await
}
