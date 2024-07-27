use chrono::Utc;
use salvo::*;
use serde::Deserialize;
use serde::Serialize;
use tracing::*;

/// BmbpDevBpmn 存储流程
pub struct BmbpDevBpmn {
    // 关联资源- 所属应用、所属模块、所属功能
    res_id: Option<String>,
    // 流程名称
    bpmn_name: Option<String>,
    // 流程描述
    bpmn_desc: Option<String>,
    // 流程缩略图
    bpmn_img: Option<String>,
    // 流程配置
    bpmn_meta: Option<String>,
}
