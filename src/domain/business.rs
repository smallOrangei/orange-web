use lombok::Data;
use serde::{Deserialize, Serialize};

/**
 * @author loveweni
 * @organization: orange
 * @create 2025-02-25 19:40:08
 * @description 实现序列化、反序列化特征、(get、set、有参构造器、无参构造器、equals_and_hash_code、to_string)
 */

#[derive(Serialize, Deserialize, Data)]
pub struct Business {
    name: String,
    revenue: u32,
    profit: i32,
}