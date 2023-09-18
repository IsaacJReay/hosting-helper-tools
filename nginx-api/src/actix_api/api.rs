use super::{
    super::{init_migration, Ed25519, User},
    obj_error::ActixCustomError,
    HttpResponse,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    Error, HttpRequest,
};
use libnginx_wrapper::{
    dbtools::crud::select_all_from_tbl_nginxconf,
    http_server::{nginx_obj::NginxObj, remake_ssl, remove_nginx_conf, target_site::TargetSite},
};

// pub async fn default_route() -> Result<HttpResponse, ActixCustomError> {
//     Err(ActixCustomError::new( 404, String::from("Not Found")))
// }

#[post("/login")]
async fn login(
    token_signer: Data<actix_jwt_auth_middleware::TokenSigner<User, Ed25519>>,
    loggging_user: Json<User>,
) -> Result<HttpResponse, ActixCustomError> {
    match dotenv::var("USERNAME").unwrap() == loggging_user.username
        && dotenv::var("PASSWORD").unwrap() == loggging_user.password
    {
        true => Ok(()),
        false => Err(ActixCustomError::new(401, String::from("Unauthorised"))),
    }?;

    let acc_token = token_signer
        .create_signed_token(&loggging_user, chrono::Duration::hours(1))
        .unwrap();
    let ref_token = token_signer
        .create_signed_token(&loggging_user, chrono::Duration::days(1))
        .unwrap();
    Ok(HttpResponse::Ok()
        .json(serde_json::json!({"refresh_token": ref_token, "access_token": acc_token})))
}

#[get("/nginx/list")]
pub async fn get_nginx_list() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(select_all_from_tbl_nginxconf()))
}

#[post("/nginx/add")]
pub async fn post_add_nginx(args: Json<NginxObj>) -> Result<HttpResponse, ActixCustomError> {
    let args = args.into_inner();

    match args.finish() {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomError::new(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[put("/nginx/update/{server_name}")]
pub async fn put_update_target_site(
    req: HttpRequest,
    target_site: Json<TargetSite>,
) -> Result<HttpResponse, ActixCustomError> {
    let server_name = req.match_info().get("server_name").unwrap();
    let target_site = target_site.into_inner();

    match NginxObj::update_target(server_name, target_site) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomError::new(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/cert/force/{server_name}")]
pub async fn post_force_cert(req: HttpRequest) -> Result<HttpResponse, ActixCustomError> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomError::new(
            400,
            String::from("Missing Server Name"),
        )),
    }?;

    match remake_ssl(server_name) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomError::new(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/migration/force")]
pub async fn post_force_migration() -> Result<HttpResponse, Error> {
    init_migration(true);
    Ok(HttpResponse::Ok().finish())
}

#[delete("/nginx/delete/{server_name}")]
pub async fn delete_remove_nginx(req: HttpRequest) -> Result<HttpResponse, ActixCustomError> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomError::new(
            400,
            String::from("Missing Server Name"),
        )),
    }?;

    match remove_nginx_conf(server_name.as_ref()) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomError::new(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().finish())
}
