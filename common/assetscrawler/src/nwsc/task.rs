use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;

pub use resd::Role;

use crate::nwsc::{self, resd};
use crate::resp::UrlBuild as UB;

pub fn meta_init() -> HashMap<String, Vec<String>> {
    let mut h = HashMap::new();
    let m = [[
        "精灵",
        "名称,ID,系列名称,精灵定位,性别,状态,攻击,防御,特攻,特防,速度,体力,总能力值,属性",
    ]];
    for [k, t] in m {
        let v = t.split(",").map(String::from).collect::<Vec<String>>();
        h.insert(k.to_string(), v);
    }
    h
}

pub trait Querysys{
    fn sysqp(data: HashMap<String, Vec<String>>) -> QueryParam;
}

impl Querysys for Role {
    fn sysqp(data: HashMap<String, Vec<String>>) -> QueryParam {
    let mode = Nmode::new("精灵", data);
    let mut para = QueryParam::new(mode);
    para.add_query("ID")
        .add_query("名称")
        .add_query("属性")
        .add_query("性别")
        .add_query("攻击")
        .add_query("防御")
        .add_query("特攻")
        .add_query("特防")   
        .add_query("速度")
        .add_query("体力")     
        .add_query("总能力值")
        .add_query("精灵定位");
    para
    }
    
}

pub struct Tasksys<T> {
    pub table: T,
    pub qp: QueryParam,
}
impl<T: Default + Querysys> Tasksys<T> {
    pub fn from_data(data: HashMap<String, Vec<String>>) -> Self {
        Self { table: T::default(), qp: T::sysqp(data) }
    }
}

#[derive(Clone)]
pub struct Nmode {
    name: String,
    coln: Vec<String>,
}
impl Nmode {
    pub fn new(name: &str, data: HashMap<String, Vec<String>>) -> Self {
        let n = name.to_string();
        let c = data.get(name).expect("No such key");
        Self {
            name: n,
            coln: c.clone(),
        }
    }
}

#[derive(Clone)]
pub struct QueryParam {
    init: String,
    mode: Nmode,
}
impl QueryParam {
    pub fn new(mode: Nmode) -> Self {
        let i = format!("#ask:[[分类:{}]]", mode.name);
        Self { init: i, mode }
    }

    pub fn add_query(&mut self, p: &str) -> &mut Self {
        if self.mode.coln.contains(&p.to_string()) {
            self.init.push_str(&format!("|?{}", p));
        }
        self
    }
    pub fn add_param(&mut self, sort: &str, lim: usize, ofs: usize) -> &mut Self {
        let rsort = if !self.mode.coln.contains(&sort.to_string()) {
            self.mode.coln[0].clone()
        } else {
            sort.to_string()
        };
        self.init.push_str(&format!(
            "|sort={}|order=desc|limit={}|offset={}|format=table|link=none",
            rsort, lim, ofs
        ));
        self
    }
    pub fn go(self) -> String {
        format! {"{{{{{}}}}}",self.init}
    }
}

pub fn url_init()->UB{
    let mut ub = UB::new("https://wiki.biligame.com/seer".to_string());
    ub.add_path("/api.php".to_string()).use_path(1);
    ub
}

pub async fn api_url(text: &str) -> String {
    let ubc = url_init().goclient();
    bech_api_url(text, &ubc).await}


pub async fn bech_api_url(text: &str,ubc:&Client) -> String {
    println!("{text}");
    let mut ub = url_init();
    let url = &ub.gourl();
    let mut mainurl = url.clone();
    mainurl
        .query_pairs_mut()
        .append_pair("format", "json")
        .append_pair("action", "parse")
        .append_pair("text", text)
        .append_pair("contentmodel", "wikitext");
    let text = nwsc::ayasurl(mainurl.as_str(), &ubc).await;
    text
}
pub fn api_ags(text: String) -> Vec<Vec<String>> {
    let v: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();

    let txt = v["parse"]["text"]["*"].as_str().unwrap();
    std::fs::write("response.html", txt).unwrap();
    let doc = scraper::Html::parse_document(&txt);
    let setr = scraper::Selector::parse("tr").unwrap();
    let seth = scraper::Selector::parse("td").unwrap();
    let body = doc.select(&setr).collect::<Vec<_>>();
    let mut optvec: Vec<Vec<String>> = Vec::new();
    for i in body {
        let rows = i.select(&seth).collect::<Vec<_>>();
        let p = rows
            .iter()
            .map(|i| i.text().next().map(String::from).unwrap_or_default())
            .collect::<Vec<_>>();
        if !p.is_empty(){optvec.push(p)};
    }
    optvec
}
