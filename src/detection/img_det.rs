#![allow(non_snake_case)]

// use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use anyhow::{anyhow, Result};
use tracing::{debug, info};

use crate::ImgData;

pub async fn detect(input: String, stream: &mut TcpStream) -> Result<ImgData> {
    // 创建发送数据结构体
    let mut input_data = ImgData::default();
    input_data.img_path = input;
    debug!("input data was {:#?}", input_data);
    let send_info = serde_json::to_vec(&input_data)?;
    // 写入数据
    stream.writable().await?;
    stream.try_write(&send_info)?;
    info!("Image data sent.");
    // 读取数据
    stream.readable().await?;
    let mut buffer = Vec::with_capacity(1024);
    loop {
        match stream.try_read_buf(&mut buffer) {
            Ok(0) => {
                info!("Can't read any data.");
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
            Ok(n) => {
                debug!("Buffer size: {}", n);
                let output = String::from_utf8(buffer)?;
                debug!("response was {:#?}", output);
                let output_data: ImgData = serde_json::from_str(&output)?;
                info!("Got data {:#?}", output_data);
                stream.flush().await?;
                return Ok(output_data);
            }
        };
    }
    stream.flush().await?;
    Err(anyhow!("Can't read any data."))
}
