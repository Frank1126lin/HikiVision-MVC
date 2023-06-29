use anyhow::{anyhow, Result};
use chrono::prelude::*;
use opencv::{core::*, imgcodecs, imgproc};
use std::{fs, path::Path};
use tracing::{debug, info};
use crate::ImgData;


pub async fn img_proc_and_transf(img: &Mat) -> Result<String> {
    let mut output = Mat::default();
    imgproc::cvt_color(img, &mut output, imgproc::COLOR_BGR2GRAY, 0)?;
    debug!("Image process finished.");
    // let mut img_result = Mat::default();
    // imgproc::adaptive_threshold(
    //     &img_gray,
    //     &mut img_result,
    //     10.,
    //     imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
    //     imgproc::THRESH_BINARY,
    //     3,
    //     0.
    // )?;
    // 写入图像到文件,后期文件名需要根据时间和产品定义
    let file_name = "Z:/capture.jpg";
    let write_para = vec![imgcodecs::IMWRITE_JPEG_QUALITY, 96];
    let write_result = imgcodecs::imwrite(file_name, &output, &write_para.into())?;
    if write_result {
        debug!("File write to {}", file_name);
        Ok(file_name.to_string())
    } else {
        info!("Image Write Failed.");
        Err(anyhow!("Write Failed!"))
    }
}


pub async fn img_save(mut input: Mat, tar_info: ImgData, save_dir: &str) -> Result<()> {
    // 设置图像文件名时间格式
    let time_fmt = "%Y_%m_%d_%H_%M_%S_%3f";
    let date_fmt = "%Y_%m_%d";
    let now_time: DateTime<Local> = Local::now();
    let dt = now_time.format(time_fmt); // 日期时间
    let d = now_time.format(date_fmt); // 日期
                                       // 设置产品、序列号保存格式，后期可以通过接口从其他系统返回或自定义文件读取
    let product_id = "271001";
    let index = "1001";
    let img_name_bmp = format!("{}-{}-{}.{}", product_id, index, dt, "bmp");
    let img_name_jpg = format!("{}-{}-{}.{}", product_id, index, dt, "jpg");
    //设置保存路径
    let save_dir_src = format!("{}\\{}\\{}", save_dir, d, "bmp");
    let save_dir_ret = format!("{}\\{}\\{}", save_dir, d, "result");
    // 判断存图路径，如果没有，连续创建
    if !Path::new(&save_dir_src).exists() {
        info!("Dir not exist, Creating......");
        fs::create_dir_all(Path::new(&save_dir_src))?;
    }
    if !Path::new(&save_dir_ret).exists() {
        info!("Dir not exist, Creating......");
        fs::create_dir_all(Path::new(&save_dir_ret))?;
    }

    // 如果没有检测到目标，则不保存图像；可单独设置
    if tar_info.target_list.is_empty() {
        info!("Result OK");
        let bmp_save_path = format!("{}\\{}", save_dir_src, img_name_bmp);
        imgcodecs::imwrite(&bmp_save_path, &input, &vec![].into())?;
        info!("Source image has been saved to {}", bmp_save_path);
    } else {
        info!("Result NG");
        // 如检测到目标，保存原图和对应的结果图
        // 设置源图像绝对路径并写入原图
        let bmp_save_path = format!("{}\\{}", save_dir_src, img_name_bmp);
        imgcodecs::imwrite(&bmp_save_path, &input, &vec![].into())?;
        info!("Source image has been saved to {}", bmp_save_path);
        // 设置结果图像绝对路径并写入结果图
        let jpg_save_path = format!("{}\\{}", save_dir_ret, img_name_jpg);
        // 根据ImageData获取图像上目标框并画框
        for tar in &tar_info.target_list {
            let p1 = Point::new(tar.box_pos[0], tar.box_pos[1]);
            let p2 = Point::new(tar.box_pos[2], tar.box_pos[3]);
            imgproc::rectangle(
                &mut input,
                Rect::from_points(p1, p2),
                Scalar::new(0., 0., 255., 0.),
                3i32,
                imgproc::LINE_8,
                0i32,
            )?;
            // 置信度及标签类型涉及put_text，后续再根据需要添加
        }
        // 写入结果图像
        imgcodecs::imwrite(&jpg_save_path, &input, &vec![].into())?;
        info!("Result image has been saved to {}", jpg_save_path);
    }
    Ok(())
}
