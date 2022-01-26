use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Value};
use anyhow::{anyhow, Error};
use std::env;

#[derive(Serialize, Deserialize)]
struct Res {
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Record {
    id: String,
    name: String,
    value: String,
    updated_on: String,
    line_id: String,
}

/// 从环境变量中读取domain、sub_domain、token
fn main() -> Result<(), Error> {
    let domain = env::var("dnspod_domain")?;
    let sub_domain = env::var("dnspod_subdomain")?;
    let token = env::var("dnspod_token")?;
    let mut latest_ip = "".to_string();
    loop {
        let current_ip = current_ip();
        if let Ok(current_ip) = current_ip {
            // let current_ip = "127.0.0.1".to_string();
            println!("current ip = {}", current_ip);
            if current_ip != latest_ip {
                if let Some(record) = get_record(&domain, &sub_domain, &token) {
                    modify_record(&current_ip, &record, &token, &domain);
                }
                latest_ip = current_ip;
            }
        }
        sleep(Duration::from_secs(30))
    }
}

fn current_ip() -> Result<String, Error> {
    let result = reqwest::blocking::get("https://sg.gcall.me/ip");
    match result {
        Ok(ip) => match ip.text() {
            Ok(text) => Ok(text),
            Err(e) => Err(anyhow!(e))
        },
        Err(e) => Err(anyhow!(e))
    }
}

fn get_record(domain: &str, sub_domain: &str, token: &str) -> Option<Record> {
    let mut params = HashMap::new();
    params.insert("login_token", token);
    params.insert("format", "json");
    params.insert("error_on_empty", "no");
    params.insert("lang", "en");
    params.insert("domain", domain);
    params.insert("sub_domain", sub_domain);

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://dnsapi.cn/Record.List")
        .form(&params)
        .send();
    if let Ok(res) = res {
        if let Ok(text) = res.text() {
            println!("响应为 {:?}", text);
            let result: serde_json::Result<Res> = serde_json::from_str(&text);
            if let Ok(res) = result {
                if res.records.len() == 1 {
                    println!("当前记录的情况 {:?}", res.records[0]);
                    return Some((&res.records[0]).clone());
                }
            }
        }
    }
    return None;
}

fn modify_record(current_ip: &String, record: &Record, token: &str, domain: &str) {
    if &record.value != current_ip {
        let client = reqwest::blocking::Client::new();
        let mut params = HashMap::new();
        params.insert("login_token", token);
        params.insert("format", "json");
        params.insert("error_on_empty", "no");
        params.insert("lang", "en");
        params.insert("domain", domain);
        params.insert("sub_domain", &record.name);
        params.insert("record_id", &record.id);
        params.insert("record_line_id", &record.line_id);
        params.insert("value", current_ip);
        let res = client.post("https://dnsapi.cn/Record.Ddns")
            .form(&params)
            .send();
        if let Ok(res) = res {
            let text = res.text();
            if let Ok(text) = text {
                println!("调用结果： {}", text);
            }
        }
    }
}
