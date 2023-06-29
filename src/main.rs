#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Result};
use opencv::core::*;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tracing::{debug, info};
use MVCam11::*;

#[tokio::main]
async fn main() -> Result<()> {
    //日志初始化
    let file_appender = tracing_appender::rolling::daily("./log", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    logging(non_blocking);
    // 相机取图
    let mut device_list = MV_CC_DEVICE_INFO_LIST::default();
    let ntype = MV_GIGE_DEVICE;
    // 枚举设备
    let enum_ret = enum_devices(ntype, &mut device_list)?;
    if enum_ret != MV_OK {
        debug!("EnumDevices failed.");
        return Err(anyhow!("EnumDevices failed."))
    } else {
        debug!("Enum Deivices success.");
    }
    //判断设备是否可达
    let d = device_list.pDeviceInfo[0];
    let aces_ret = is_device_accessible(d, 3u32)?;
    if !aces_ret {
        debug!("Access is unavilable.");
        return Err(anyhow!("Access is unavilable."))
    } else {
        debug!("Access is success.");
    }
    // 创建句柄
    let mut handle = 0 as *mut ::std::os::raw::c_void;
    let handle_ret = create_handle(&mut handle, d)?;
    if handle_ret != MV_OK {
        debug!("Create handle failed.");
        return Err(anyhow!("Create handle failed"))
    } else {
        debug!("Create handle success.");
    }
    // 打开相机
    let nam = MV_ACCESS_Exclusive;
    let nsok = 0;
    let open_ret = open(handle, nam, nsok)?;
    if open_ret != MV_OK {
        debug!("Open device failed.");
        return Err(anyhow!("Open device failed."))
    } else {
        debug!("Open device success");
    }


    // 设置相机各项参数，如照片格式、曝光时间、增益、伽马等。
    // 参考软件中各项设置
    //设置相机曝光时间
    let expose_time = 86000 as f32;
    let exp = std::ffi::CString::new("ExposureTime")?;
    let exp_key = exp.as_ptr();
    let expset_ret = set_float_value(handle, exp_key, expose_time)?;
    if expset_ret != MV_OK {
        debug!("Setting expose time Failed.");
        return Err(anyhow!("Setting expose time Failed."))
    } else {
        debug!("Setting expose time Success.")
    }
    // 设置相机取图格式为BGR8
    let img_format = MvGvspPixelType_PixelType_Gvsp_BGR8_Packed as u32;
    let fmtset_ret = set_pixel_format(handle, img_format)?;
    if fmtset_ret != MV_OK {
        debug!("Setting format Failed.");
        return Err(anyhow!("Setting format Failed."))
    } else {
        debug!("Setting format Success.")
    }

    // 开始取流
    let grabbing_ret = start_grabbing(handle)?;
    if grabbing_ret != MV_OK {
        debug!("Grabbing failed.");
        return Err(anyhow!("Grabbing failed."))
    } else {
        debug!("Start grabbing stream success.")
    }
    // 开始循环听取网络信号，并抓取图像，以及释放缓存
    let plc_addr = "127.0.0.1:9527";
    let ai_addr = "127.0.0.2:7966";
    let mut plc_stream = TcpStream::connect(plc_addr).await?;
    let mut ai_stream = TcpStream::connect(ai_addr).await?;
    plc_stream.try_write(b"Hello, PLC")?;
    loop {
        let mut buf = Vec::with_capacity(1024);
        plc_stream.readable().await?;
        match plc_stream.try_read_buf(&mut buf) {
            Ok(0) => {
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                debug!("PLC stream error {}", e);
                return Err(e.into());
            }
            Ok(_n) => {
                info!("Got signal: {:?}", buf);
                if buf[0] == 1 {
                    img_get_proc_and_detect(handle, &plc_stream, &mut ai_stream).await?;
                } else if buf[0] == 0 {
                    info!(">>>>>>>>Program Quit<<<<<<<<");
                    break;
                } else {
                    match std::str::from_utf8(&buf) {
                        Ok("1") => {
                            plc_stream.writable().await?;
                            plc_stream.try_write(b"You send string 1.")?;
                            debug!("PLC send string 1.");
                        }
                        Ok("0") => {
                            plc_stream.writable().await?;
                            plc_stream.try_write(b"You send string 0.")?;
                            debug!("PLC send string 0.");
                        }
                        Err(e) => {
                            plc_stream.writable().await?;
                            plc_stream.try_write(b"Got Err")?;
                            debug!("PLC Got Err {}.", e);
                        }
                        _ => {
                            plc_stream.writable().await?;
                            plc_stream.try_write(b"Wrong signal.")?;
                            debug!("Wrong signal.");
                        }
                    }
                }
            }
        }
    }
    plc_stream.flush().await?;
    plc_stream.shutdown().await?;
    ai_stream.flush().await?;
    ai_stream.shutdown().await?;
    // 停止取流
    let stop_ret = stop_grabbing(handle)?;
    if stop_ret != MV_OK {
        debug!("Stop grabbing failed.");
        return Err(anyhow!("Stop grabbing failed."));
    } else {
        debug!("Stop grabbing success.");
    }
    // 关闭相机
    let close_ret = close(handle)?;
    if close_ret != MV_OK {
        debug!("Close device failed.");
        return Err(anyhow!("Close device failed."));
    } else {
        debug!("Close device success.")
    }
    // 销毁句柄
    let destroy_ret = destroy_handle(handle)?;
    if destroy_ret != MV_OK {
        debug!("Destroy handle failed.");
        return Err(anyhow!("Destroy handle failed."));
    } else {
        debug!("Destroy handle success.")
    }
    Ok(())
}

// 图像抓取、处理、以及推理函数
async fn img_get_proc_and_detect(
    handle: *mut ::std::os::raw::c_void,
    plc_stream: &TcpStream,
    ai_stream: &mut TcpStream,
) -> Result<()> {
    let mut pframe = MV_FRAME_OUT::default();
    let ms = 1000;
    let frame_ret = get_image_buffer(handle, &mut pframe, ms)?;
    if frame_ret != MV_OK {
        debug!("Get frame failed");
        return Err(anyhow!("Get frame failed"));
    } else {
        debug!(
            "Get frame success: Width: {}, Height: {}",
            pframe.stFrameInfo.nWidth, pframe.stFrameInfo.nHeight,
        );
    }
    debug!("First unsafe operation.");
    // 图像指针转换，用于OpenCV保存
    let raw_data = unsafe {
        std::mem::transmute::<*mut ::std::os::raw::c_uchar, *mut ::std::os::raw::c_void>(
            pframe.pBufAddr,
        )
    };
    debug!("Second unsafe operation.");
    // 使用OpenCV保存图像
    let img_h = pframe.stFrameInfo.nHeight as i32;
    let img_w = pframe.stFrameInfo.nWidth as i32;
    let src_img = unsafe { Mat::new_rows_cols_with_data(img_h, img_w, CV_8UC3, raw_data, 0) }?;
    info!("Source Image size: {:#?}", src_img.size()?);
    // 图像处理及转换
    // 220518: 相机设置的取图格式不对，改为BGR8即可，程序已加入自动设置
    let file_name = img_proc_and_transf(&src_img).await?;
    debug!("Image Process Finished.");
    // 连接AI服务推理图像
    debug!("Start detect {}", file_name);
    let output_data = detect(file_name, ai_stream).await?;
    debug!("Detect finished.");
    info!("The image detect result is {:?}", output_data.img_result);
    match output_data.img_result.as_str() {
        "OK" => {
            plc_stream.writable().await?;
            plc_stream.try_write(b"OK")?;
        }
        "NG" => {
            plc_stream.writable().await?;
            plc_stream.try_write(b"NG")?;
        }
        _ => {
            plc_stream.writable().await?;
            plc_stream.try_write(b"No Result.")?;
        }
    }
    // // 后期需要结合传统视觉算法和AI推理算法共同确定图像结果
    // // 异步/多线程写入原图及处理后的结果图
    // // 会用到 原图src_img, 处理结果数据output_data
    let save_img = src_img.clone();
    let out_data = output_data.clone();
    let save_dir = "./images";
    // img_save(save_img, out_data, save_dir).await?;
    tokio::spawn(async move {
        img_save(save_img, out_data, save_dir).await
    });
    //写入完成
    let free_buffer = free_image_buffer(handle, &mut pframe)?;
    if free_buffer != MV_OK {
        debug!("Free Frame Failed");
        return Err(anyhow!("Free Frame Failed"));
    } else {
        debug!("Free Frame Success.");
    }
    Ok(())
}
