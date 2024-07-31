use std::fs::File;
use std::io::Write;
use std::{env, fs};
use std::error::Error;
use std::path::{Path, PathBuf};

use dotenv::dotenv;
use rocket::{get, launch};
use tera::{Context, Tera};
use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use zip::ZipArchive;

use crate::domain_config::Websites;

mod domain_config;


fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    dotenv().ok();
}


async fn save_config(domain_id: i32) -> Result<bool, Box<dyn Error>> {
    let nginx_config_path = env::var("NGINX_CONFIG_PATH")?;
    let www_path = env::var("WWW_PATH")?;


    // 获取域名配置数据
    let response = reqwest::get(format!("https://console.d-l.ink/api/domain/getAgentConfig?id={}", domain_id)).await.expect("config load fail");
    let domain_config = response.json::<domain_config::DomainConfig>().await?;

    // NGINX 配置文件目录
    let www_path = Path::new(&www_path);
    let website_path = www_path.join(domain_config.domain.as_ref().unwrap());
    // 删除目录所有文件
    if website_path.exists() {
        fs::remove_dir_all(&website_path)?;
    }
    for website in domain_config.websites.as_ref().unwrap() {
        if website.websites_type.as_ref().unwrap().eq("LANDING") {
            download_website(&website_path, &website).await;
        }else if website.websites_type.as_ref().unwrap().eq("LINK") {
            generate_path_dir(&website_path, &website).await;

        }
    }

    // 加载模板引擎
    let tera = Tera::new("templates/*")?;
    let mut context = Context::new();
    context.insert("config", &domain_config);

    // 渲染模板
    let config_content = tera.render("nginx.config.j2", &context)?;

    // 写入到文件
    let config_path = Path::new(&nginx_config_path);
    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    info!("写入配置文件 -> {}", config_path.join(format!("{}.conf", domain_config.domain.as_ref().unwrap())).to_str().unwrap());
    fs::write(config_path, config_content)?;

    // 重启NGINX
    info!("检测NGINX配置文件是否正确");
    let nginx_test = std::process::Command::new("nginx").arg("-t").output().expect("nginx test fail");
    info!("{}", String::from_utf8_lossy(&nginx_test.stdout));
    // 没有问题则重启
    if nginx_test.status.success() {
        info!("重启NGINX");
        let nginx_reload = std::process::Command::new("nginx").arg("-s").arg("reload").output().expect("nginx reload fail");
        info!("{}", String::from_utf8_lossy(&nginx_reload.stdout));
    }
    return Ok(true);
}


fn unzip_file(zip_file: &PathBuf, output_dir: &Path) {
    let zip_file_name = zip_file.file_name().as_ref().unwrap().to_str().unwrap();
    let zip_file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(zip_file).unwrap();
    // 获取压缩包文件名

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut file_name = file.name().to_owned();
        // ZIP迭代器的第一个文件名是目录名，需要去掉
        file_name.remove(0);
        let removal_parent_dir = format!("{}/", zip_file_name.strip_suffix(".zip").unwrap());
        info!(file_name);
        info!(removal_parent_dir);
        if file_name.starts_with(&removal_parent_dir) {
            file_name = file_name.strip_prefix(&removal_parent_dir).unwrap().to_owned();
        }
        let out_path = output_dir.join(file_name);
        info!("解压文件 -> {}", out_path.to_str().unwrap());

        if file.is_dir() {
            std::fs::create_dir_all(out_path).unwrap();
        } else {
            let mut outfile = File::create(out_path).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

async fn generate_path_dir(domain_path: &Path, website: &Websites) {
    let mut website_path = domain_path.join(website.id.as_ref().unwrap().to_string());
    website.path.as_ref().unwrap().split("/").for_each(|path| {
        website_path = domain_path.join(path);
    });
    if !website_path.exists() {
        fs::create_dir_all(&website_path).expect("dir create fail");
    }
        // 写入配置文件
        let config_path = website_path.join("config.json");
        let config_content = serde_json::to_string(website).expect("json serialize fail");
        fs::write(config_path, config_content).expect("file write fail");

        // 写入index.php文件
        let index_path = website_path.join("index.php");
        let index_content = "<?php require_once '/www/wwwroot/engine.php' ?>";
        fs::write(index_path, index_content).expect("file write fail");
}
async fn download_website(domain_path: &Path, website: &Websites) {
    // 处理路径
    let www_lib_url = env::var("WEB_LIB_PATH").expect("env CONFIG_PATH not config");
    let landing_uuid = website.landing.as_ref().unwrap().uuid.as_ref().unwrap();
    let resource_url = format!("{}{}.zip", www_lib_url, landing_uuid);
    let mut website_path = domain_path.join(website.id.as_ref().unwrap().to_string());
    website.path.as_ref().unwrap().split("/").for_each(|path| {
        website_path = domain_path.join(path);
    });
    info!("下载网站 {} -> {}",resource_url, website_path.to_str().unwrap());
    if !website_path.exists() {
        fs::create_dir_all(&website_path).expect("dir create fail");
    }
    let response = reqwest::get(&resource_url).await.expect("download fail");

    let zip_path = website_path.join(format!("{}.zip", landing_uuid));
    let mut file = File::create(&zip_path).expect("file create fail");
    let content = response.bytes().await.expect("download fail");
    file.write_all(&content).expect("file write fail");
    info!("下载完成 -> {}", website_path.to_str().unwrap());

    // 解压
    info!("解压 -> {}", zip_path.to_str().unwrap());
    unzip_file(&zip_path, &website_path);

    // 写入配置文件
    let config_path = website_path.join("config.json");
    let config_content = serde_json::to_string(website).expect("json serialize fail");
    fs::write(config_path, config_content).expect("file write fail");
}


#[get("/deploy/domain?<domain_id>")]
async fn deploy_domain(domain_id: i32) -> &'static str {
    info!("开始部署");
    match save_config(domain_id).await {
        Ok(_) => "OK",
        Err(_) => "FAIL"
    }
}

#[launch]
fn rocket() -> _ {
    init();
    rocket::build()
        .mount("/", rocket::routes![deploy_domain])
}
