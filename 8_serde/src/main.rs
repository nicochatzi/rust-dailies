#![allow(dead_code)]

use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "plans.json";

#[derive(Debug, Serialize, Deserialize)]
struct OldPlan {
    id: String,
    price: f32,
}

fn serialize_with_debug() {
    let plan = OldPlan {
        id: String::from("123-123"),
        price: 1.0,
    };
    let plan_str = format!("plan.id: {}, plan.price: {}", plan.id, plan.price);
    println!("{}", plan_str);
    println!("{:?}", plan);
    println!("{:#?}", plan);
}

fn serialize_with_serde() {
    let plan = OldPlan {
        id: String::from("123-123"),
        price: 1.0,
    };
    let plan_str = serde_json::to_string(&plan).unwrap();
    println!("{}", plan_str);
    std::fs::write(FILE_NAME, plan_str).unwrap();
}

fn deserialize_with_serde() {
    let plan_str = std::fs::read(FILE_NAME).unwrap();
    let plan: OldPlan = serde_json::from_slice(&plan_str).unwrap();
    println!("{:#?}", plan);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plans {
    plans: Vec<Plan>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    id: String,
    name: String,
    description: String,
    hidden: bool,
    free: bool,
    warehouse_access: bool,
    formats: Vec<String>,
    software_ids: Vec<String>,
    hubspot_name: Option<String>,
    hubspot_date_registered_name: Option<String>,
    price: Option<Price>,
    term: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Price {
    gbp: f64,
    eur: f64,
    usd: f64,
    cad: f64,
}

fn main() {
    // run this with the following watch command
    // so it will ignore any changes to *.json files
    // otherwise writing to a *.json file will
    // trigger a re-build!
    // cargo watch -c -x run -i *.json
    let plan_str = reqwest::blocking::get("https://api.ampifymusic.com/plans")
        .unwrap()
        .text()
        .unwrap();
    let plans: Plans = serde_json::from_str(&plan_str).unwrap();
    for plan in plans.plans {
        println!("{:#?}", plan);
    }
    std::fs::write(FILE_NAME, plan_str).unwrap();
}
