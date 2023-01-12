use actix_web::web;

use self::get_user_list::list_users;
use self::post_user_signup::signup;
use self::post_user_login::login;

pub mod sql_user;

pub mod get_user_list;
pub mod post_user_signup;
pub mod post_user_login;

pub fn user_service_config(cfg: &mut web::ServiceConfig) {    
    cfg
        .service(list_users)
        .service(signup)
        .service(login);
}