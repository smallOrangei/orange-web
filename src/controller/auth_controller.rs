use rocket::{delete, get, post, put};
use crate::domain::business::Business;

/**
 * @author loveweni
 * @organization: orange
 * @create 2025-02-25 19:41:16
 */

/*
 * get方法
 * @param value_str 字符串参数
 * @param value_u32 u32
 * @param value_i32 i32
 * @return String
 */
#[get("/get/<value_str>/<value_u32>/<value_i32>")]
pub fn get(value_str: String, value_u32: u32, value_i32: i32) -> String {
    serde_json::to_string(&Business::new(value_str, value_u32, value_i32)).unwrap()
}

/* post方法 */
#[post("/post")]
pub fn post() -> String {
    serde_json::to_string(&Business::new(String::from("ABC"), 1000, 100)).unwrap()
}

/* put方法 */
#[put("/put")]
pub fn put() -> String {
    serde_json::to_string(&Business::new(String::from("ABC"), 1000, 100)).unwrap()
}

/* delete方法 */
#[delete("/delete")]
pub fn delete() -> String {
    serde_json::to_string(&Business::new(String::from("ABC"), 1000, 100)).unwrap()
}