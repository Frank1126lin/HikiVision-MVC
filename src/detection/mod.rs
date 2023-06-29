mod img_det;
pub use img_det::detect;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImgData {
    pub img_name: String,
    pub img_path: String,
    pub img_result: String,
    pub target_list: ::std::vec::Vec<TargetInfo>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetInfo {
    pub box_pos: [i32; 4],
    pub box_conf: f32,
    pub label_type: u32,
}

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
