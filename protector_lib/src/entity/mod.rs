use serde::{Deserialize, Serialize};

/// User 用户结构
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub user_name: String,
    /// 用户秘钥
    pub user_key: String,
    /// 用户头像地址
    pub img_head: String,
}

/// Message 消息结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// 消息ID
    pub mes_id: String,
    /// 发送人昵称
    pub send_user_name: String,
    /// 发送人ID
    pub send_user_id: String,
    /// 消息来源 0自己 1对方
    pub source: u8,
    /// 消息时间
    pub times: u64,
}

/// FriendList 好友列表结构
pub struct FriendList {
    /// 好友ID
    pub friend_id: String,
    /// 好友昵称
    pub friend_name: String,
    /// 备注
    pub comment: String,
    /// 用户头像地址
    pub img_head: String,
}