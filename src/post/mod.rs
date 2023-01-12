use actix_web::web;
use self::post_post_create::create;
use self::delete_post_delete::delete;
use self::get_post_list::list_posts;

pub mod sql_text_post;

pub mod post_post_create;
pub mod delete_post_delete;
pub mod get_post_list;

pub fn post_service_config(cfg: &mut web::ServiceConfig) {    
    cfg
        .service(list_posts)
        .service(create)
        .service(delete);
}