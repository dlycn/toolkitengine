mod dbmg;
mod nwsc;
mod resp;

use std::collections::HashMap;

use dbmg::{Dbmbuilder as DMB};
use nwsc::task;

#[tokio::main]
async fn main() {
    let path = "./assets/db/assets.db";
    let mut dbm: DMB = DMB::connect(String::from(path)).unwrap();
    let data = task::meta_init();
    let bugurl = task::url_init();
    let ubapi: &resp::UrlBuild =bugurl.api.get("seer.api").unwrap();
    let ubres: &resp::UrlBuild =bugurl.res.get("seer.flash.res").unwrap();
    let lim = 1000;
    let ofs = 0;
    let ubc = ubres.goclient();
    let syst = task::Tasksys::<task::PetSkin>::from_data(data);
    
    use futures::StreamExt;

    let mut stmt = dbm.manager
    .prepare("SELECT id FROM PetData").expect("id get wrong");
    let iter =stmt.query_map([], |row|{row.get::<usize,u32>(0)}).unwrap();
    let url = "https://wiki.biligame.com/seer/%E7%B2%BE%E7%81%B5:".to_string();

    let mut offsets: Vec<u32> = iter.map(|i|i.unwrap()).collect::<Vec<u32>>();
    let ofs = &offsets[0..5];

    let results  = futures::stream::iter(ofs).map(|i|{
                let s = &i.to_string();
                let a =&ubc;
                let mut cul = url.clone();
                cul.push_str(s);

                let mut selectormap =HashMap::new();
                selectormap.entry("#wiki-wrapper > div.seer-container > div.main-card > div.right-col > div.info-box > div.info-tab-content.show > div > div:nth-child(2) > span:nth-child(2)".into()).insert_entry("img".into());
                selectormap.entry("#appearanceSection > div.appearance-content > div.appearance-img-wrap > p".into()).insert_entry("img".into());
                selectormap.entry("#wiki-wrapper > div.related-dual-row > div:nth-child(2) > div.related-skins-grid".into()).insert_entry("img".into());
                selectormap.entry("#mw-content-text > div > div.row > div:nth-child(2) > div > div > div > div > div > div:nth-child(1) > table".into()).insert_entry( "img".into());
                selectormap.entry("#mw-content-text > div > div.row > div:nth-child(4) > div.row > div:nth-child(2)".into()).insert_entry( "img".into());
                async move {
                    let text = nwsc::ayasurl(&cul,a).await;


                    let mainout = task::res_ags(text, selectormap);
                    let out = mainout.iter().map(|ores|task::extract_attrs(ores.to_vec(), vec!["alt".into(),"src".into()])).collect::<Vec<Vec<Vec<String>>>>();
                out}
    }).buffered(20).collect::<Vec<Vec<Vec<Vec<String>>>>>().await;

    let optstr = serde_json::to_string(&results).unwrap();
    std::fs::write("opt.json", optstr).unwrap();

    //syst.out_table_db(dbm, ubapi, ofs, lim).await;


    // use futures::StreamExt;
    // let mut stmt = dbm.manager
    // .prepare("SELECT id FROM PetSkin").expect("id get wrong");
    // let iter =stmt.query_map([], |row|{row.get::<usize,u32>(0)}).unwrap();
    // let mut v =vec![];
    // for i in iter{
    //     v.push(i.unwrap())
    // }
    
    // let p = ubres.clone();

    // task::go_dlink_resource(&p, v.split_off(760), "./assets/imgs/Flashes".to_string(), "swf").await;
}




