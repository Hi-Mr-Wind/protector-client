use std::time::{SystemTime, UNIX_EPOCH};

// 自定义的初始时间戳 (2024-08-09 16:08:26)
const EPOCH: u64 = 1723190906819;
const WORKER_ID_BITS: u64 = 5;
const DATA_CENTER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;

const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_DATA_CENTER_ID: u64 = (1 << DATA_CENTER_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;
const DATA_CENTER_ID_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;

pub(crate) struct SnowflakeIdGenerator {
    worker_id: u64,
    data_center_id: u64,
    sequence: u64,
    last_timestamp: u64,
}

impl SnowflakeIdGenerator {
   pub fn new(worker_id: u64, data_center_id: u64) -> Self {
        assert!(worker_id <= MAX_WORKER_ID, "worker ID can't be greater than {} or less than 0", MAX_WORKER_ID);
        assert!(data_center_id <= MAX_DATA_CENTER_ID, "data center ID can't be greater than {} or less than 0", MAX_DATA_CENTER_ID);

        SnowflakeIdGenerator {
            worker_id,
            data_center_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    pub fn next_id(&mut self) -> u64 {
        let mut timestamp = Self::get_timestamp();

        if timestamp < self.last_timestamp {
            panic!("无法给倒转的时间生成一个ID。时间:{}", self.last_timestamp - timestamp);
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;
            if self.sequence == 0 {
                timestamp = Self::til_next_millis(self.last_timestamp);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        ((timestamp - EPOCH) << TIMESTAMP_LEFT_SHIFT) |
            (self.data_center_id << DATA_CENTER_ID_SHIFT) |
            (self.worker_id << WORKER_ID_SHIFT) |
            self.sequence
    }

    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    fn til_next_millis(last_timestamp: u64) -> u64 {
        let mut timestamp = Self::get_timestamp();
        while timestamp <= last_timestamp {
            timestamp = Self::get_timestamp();
        }
        timestamp
    }
}