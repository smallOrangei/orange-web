/* 引入的具体程序 */
use rocket::{launch, routes};
use controller::auth_controller;

/* 引入的模块 */
mod domain;
mod controller;

/**
 * @author loveweni
 * @organization: orange
 * @create 2025-02-25 19:30:08
 * @description web主入口
 */
/* 启动Rocket服务器并挂载路由和数据库连接池 */
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            auth_controller::get,
            auth_controller::post,
            auth_controller::put,
            auth_controller::delete,
        ])  // 挂载路由
}