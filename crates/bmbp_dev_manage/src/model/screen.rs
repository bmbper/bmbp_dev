use chrono::Utc;
use salvo::*;
use serde::Deserialize;
use serde::Serialize;
use tracing::*;

/// BmbpDevTable 存储表结构描述
///#[rdbc_model(BMBP_DEV_SCREEN)]
pub struct BmbpDevScreen {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 大屏名称
    screen_name: Option<String>,
    // 大屏描述
    screen_desc: Option<String>,
    // 大屏综略图
    screen_img: Option<String>,
    // 大屏配置
    screen_meta: Option<String>,
}
