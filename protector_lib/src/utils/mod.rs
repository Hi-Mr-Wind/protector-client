mod id_util;

use id_util::SnowflakeIdGenerator;

/// 获取一个ID
pub fn get_id() -> u64 {
    SnowflakeIdGenerator::new(1, 1).next_id()
}