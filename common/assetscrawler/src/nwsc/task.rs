use reqwest::Client;
use rusqlite::ToSql;
use std::collections::HashMap;

use crate::dbmg::Dbmbuilder;
use crate::dbmg::tfm::Table;
use crate::nwsc::resdata::{Resfix, Work};
use crate::nwsc::{self, resdata};
use crate::resp::UrlBuild as UB;

pub use resdata::*;
use serde::*;

use futures::stream::StreamExt;

pub fn meta_init() -> HashMap<String, Vec<String>> {
    let mut h = HashMap::new();
    let m = [[
        "精灵","名称,ID,系列名称,精灵定位,性别,状态,攻击,特攻,防御,特防,速度,体力,总能力值,属性",],[
        "皮肤","名称,ID,type,类型,状态",]];
    for [k, t] in m {
        let v = t.split(",").map(String::from).collect::<Vec<String>>();
        h.insert(k.to_string(), v);
    }
    h
}

pub trait Querysys {
    fn sysqp(data: HashMap<String, Vec<String>>) -> QueryParam;
    fn query(qp: &mut QueryParam);
}

impl Querysys for PetSkin {
    fn sysqp(data: HashMap<String, Vec<String>>) -> QueryParam {
        let mode = Nmode::new("皮肤", data);
        let para = QueryParam::new(mode);
        para
    }

    fn query(qp: &mut QueryParam) {
        qp.add_query("ID")
            .add_query("名称")
            .add_query("类型");
    }
}

impl Querysys for PetData {
    fn sysqp(data: HashMap<String, Vec<String>>) -> QueryParam {
        let mode = Nmode::new("精灵", data);
        let para = QueryParam::new(mode);
        para
    }

    fn query(qp: &mut QueryParam) {
        qp.add_query("ID")
            .add_query("名称")
            .add_query("属性")
            .add_query("性别")
            .add_query("攻击")
            .add_query("特攻")
            .add_query("防御")
            .add_query("特防")
            .add_query("速度")
            .add_query("体力")
            .add_query("总能力值")
            .add_query("精灵定位");
    }
}

pub struct Tasksys<T> {
    pub table: T,
    pub qp: QueryParam,
}
impl<T: Default + Querysys + Serialize + Work + 'static + Clone> Tasksys<T> {
    pub fn from_data(data: HashMap<String, Vec<String>>) -> Self {
        Self {
            table: T::default(),
            qp: T::sysqp(data),
        }
    }

    pub fn num_query(&self) -> QueryParam {
        let mut qp = self.qp.clone();
        qp.init.push_str("|format=count");
        qp
    }

    pub fn base_query(&self) -> QueryParam {
        let mut qp = self.qp.clone();
        T::query(&mut qp);
        qp
    }

    pub fn apply_fix(self, table: &Table<T>, dbm: &mut Dbmbuilder) {
        let resfix: Resfix = self.table.fix();

        for ((pk_col, row_col), updates) in resfix.update {
            for (id, value) in updates {
                dbm.on(table)
                    .pk(pk_col) // "id"
                    .row(row_col) // "attr"
                    .update(id.into(), [value]);
            }
        }

        // 处理删除
        for (col, id) in resfix.delete {
            // 根据你的展示，col 总是 "id"，但保留灵活性
            dbm.on(table).pk(col).delete(id.into());
        }
    }

    pub async fn go_table_url(&self, ub: &UB, lim: usize) -> Vec<Vec<String>> {
        let ubc = ub.clone().goclient();
        let numdqp = self.num_query();
        let text = numdqp.go();
        let out = bech_api_url(ub.clone(), &text, &ubc).await;
        let strs = api_str(out);
        let num: usize = strs[0].trim_end().parse().unwrap();

        println!("{}", num);

        let offsets: Vec<usize> = (0..num).step_by(lim).collect();

        let results: Vec<String> = futures::stream::iter(offsets)
            .map(|offset| {
                let mut dqp = self.base_query();
                dqp.add_param("ID", lim, offset);
                let text = dqp.go();
                let refd = &ubc;
                let rub = ub.clone();
                async move {
                    let out = bech_api_url(rub, &text, refd).await;
                    out
                }
            })
            .buffered(5)
            .collect::<Vec<_>>()
            .await;

        let resasys: Vec<Vec<String>> = results
            .iter()
            .map(|text| api_ags(text.to_string()))
            .flatten()
            .collect();

        let res: Vec<Vec<String>> = resasys
            .iter()
            .map(|txt: &Vec<String>| txt.iter().skip(1).cloned().collect())
            .filter(|row: &Vec<String>| !row.is_empty()) 
            .collect();
        
        let f = format!("Outout.json");

        tokio::fs::write(f, format!("{:?}", res)).await.unwrap();
        res
    }

    pub async fn out_table_db(self,mut dbm: Dbmbuilder,ubapi:&UB,mut ofs: usize,lim: usize){
        let api = self.go_table_url(ubapi,lim).await;
        let table = Table::new(self.table.clone());
        let max = api.len();
        dbm.transaction(true).unwrap();
        dbm.on(&table).pk("id").create();
        loop {
            let end = usize::min(ofs + lim, max);
            println!("{},{}",end,ofs);
            let iter = api[ofs..end]
                .iter()
                .flat_map(|row| row.iter().map(|s| s as &dyn ToSql));
            let params = rusqlite::params_from_iter(iter);
            dbm.on(&table).append(end - ofs, params,Option::Some(3));

        

            if end == max {
                break;
            }
            ofs += lim
        }
        self.apply_fix(&table, &mut dbm);
        dbm.transaction(false).unwrap();
    }

}

pub async fn go_dlink_resource(ubres: &UB, iter: Vec<u32>, localpath: String, format: &str) {
    let ubc = ubres.goclient();
    let result = futures::stream::iter(iter)
        .map(|v| {
            let mut o = localpath.clone();
            let c = &ubc;

            let f = format!("/{}.{}", v, format);
            let mut ub = ubres.clone();
            async move {
                let p = ub.mod_path(&f);
                o.push_str(f.as_str());
                let opt = nwsc::goresurl(c, p.main_path.as_str(), o.as_str()).await;
                (v, opt)
            }
        })
        .buffered(20)
        .collect::<Vec<(u32, bool)>>()
        .await;

    let out = result
        .iter()
        .filter_map(|s| if !s.1 { Some(s.0) } else { None })
        .collect::<Vec<u32>>();

    let f = format!("{}Outout.json",localpath);

    tokio::fs::write(f, format!("{:?}", out)).await.unwrap();
}

pub async fn go_group_resource(ubres: &UB, iter: Vec<u32>, localpath: String, format: &str) {
    let ubc = ubres.goclient();}

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

pub struct Bugurl {
    pub res: HashMap<String, UB>,
    pub api: HashMap<String, UB>,
}

impl Default for Bugurl {
    fn default() -> Self {
        Self {
            res: HashMap::new(),
            api: HashMap::new(),
        }
    }
}

pub fn url_init() -> Bugurl {
    let mut bu = Bugurl::default();
    bu.api
        .entry("seer.api".to_string())
        .insert_entry(UB::new("https://wiki.biligame.com/seer".to_string()));
    bu.api
        .get_mut("seer.api")
        .unwrap()
        .add_path("/api.php".to_string())
        .use_path(1);
    bu.res.entry("seer.h5.res".to_string()).insert_entry(UB::new(
        "https://seerh5.61.com/resource/assets/fightResource/pet".to_string(),
    ));
    
    bu.res
        .get_mut("seer.h5.res")
        .unwrap()
        .use_path(0);
    
    bu.res.entry("seer.flash.res".to_string()).insert_entry(UB::new(
        "https://seer.61.com/resource/fightResource/pet".to_string(),
    ));
    bu.res
        .get_mut("seer.flash.res")
        .unwrap()
        .add_path("/swf".to_string())
        .use_path(1);

    bu
}

pub async fn api_url(ub: UB, text: &str) -> String {
    let ubc = ub.goclient();
    bech_api_url(ub, text, &ubc).await
}

pub async fn bech_api_url(mut ub: UB, text: &str, ubc: &Client) -> String {
    println!("{text}");
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
pub fn api_out(text: String) -> String {
    let v: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();

    let txt = v["parse"]["text"]["*"].as_str().unwrap();
    std::fs::write("response.html", txt).unwrap();
    txt.to_string()
}

pub fn api_str(text: String) -> Vec<String> {
    let txt = api_out(text);
    let doc = scraper::Html::parse_document(&txt);
    let setp = scraper::Selector::parse("p").unwrap();
    let body = doc.select(&setp).collect::<Vec<_>>();
    body.iter()
        .map(|i| i.text().collect::<String>())
        .collect::<Vec<_>>()
}

pub fn api_ags(text: String) -> Vec<Vec<String>> {
    let txt = api_out(text);
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
        if !p.is_empty() {
            optvec.push(p)
        };
    }
    optvec
}

use scraper::Selector as Selector;

pub fn res_ags(text: String, selectormap: HashMap<String, String>) -> Vec<Vec<String>> {
    let l = selectormap.len();
    let mut o = vec![Vec::new(); l];          // 需要 mut 以便修改
    let d = scraper::Selector::parse("root > *").unwrap();
    let doc = scraper::Html::parse_document(&text);

    for (n, (k, v)) in selectormap.iter().enumerate() {
        let setk = Selector::parse(k.as_str()).unwrap_or_else(|_| d.clone());
        let setv = Selector::parse(v.as_str()).unwrap_or_else(|_| d.clone());

        if setk != d && setv != d {
            // 1. 先按键选择器定位元素
            for element in doc.select(&setk) {
                // 2. 在定位到的元素内部，按值选择器提取内容
                for child in element.select(&setv) {
                    let extracted = child.html();
                    o[n].push(extracted);
                }
            }
        }
    }
    o
}

pub fn extract_attrs(elements: Vec<String>, attrs: Vec<String>) -> Vec<Vec<String>> {
    let mut results = Vec::with_capacity(elements.len());

    // 构造一个选择器，用于定位包装后的根元素下的第一个子元素
    let root_selector = Selector::parse("root > *").unwrap(); // 只取第一个直接子元素

    for elem_str in elements {
        // 将标签包裹在 <root> 中，使其成为合法的文档片段
        let doc = scraper::Html::parse_document(&format!("<root>{}</root>", elem_str));
        let mut row = Vec::with_capacity(attrs.len());

        // 尝试定位到该标签
        if let Some(element) = doc.select(&root_selector).next() {
            // 提取每个属性
            for attr_name in &attrs {
                let value = element.value().attr(attr_name).unwrap_or("").to_string();
                row.push(value);
            }
        } else {
            // 如果无法解析（例如空字符串或格式错误），则填充空字符串
            row.resize(attrs.len(), String::new());
        }
        results.push(row);
    }

    results
}
