mod dbmg;
mod nwsc;
mod resp;

use dbmg::{Dbmbuilder as DMB};
use nwsc::task;

use crate::nwsc::task::Selectormap;

#[tokio::main]
async fn main() {
    let path = "./assets/db/assets.db";
    let mut dbm: DMB = DMB::connect(String::from(path)).unwrap();
    let data = task::meta_init();
    let bugurl = task::url_init();
    let mut ubapi: &resp::UrlBuild =bugurl.api.get("seer.api").unwrap();
    let ubres: &resp::UrlBuild =bugurl.res.get("seer.flash.res").unwrap();
    let lim = 1000;
    let ofs = 0;
    let ubc = ubres.goclient();
    let syst = task::Tasksys::<task::PetSkin>::from_data(data);
    
    use futures::StreamExt;

    let mut stmt = dbm.manager
    .prepare("SELECT id FROM PetData").expect("id get wrong");
    let iter =stmt.query_map([], |row|{row.get::<usize,u32>(0)}).unwrap();



    let url = ubapi.clone().use_path(2).gourl();
    let text = nwsc::ayasurl(url.as_str(),&ubc).await;
    let mut selectormap =vec![Selectormap::new("img".into())];
    selectormap[0].add("#myTabContent".into());
    let out = task::res_ags(text, selectormap);
    let opt = out.iter().map(|i|{task::extract_attrs(i.to_vec(), vec!["alt".into(),"src".into()])}).collect::<Vec<Vec<Vec<String>>>>();
    futures::stream::iter(opt[0].clone()).map(|d|{
        let (alt,src) =(d[0].clone(),d[1].clone());
        let dc =&ubc;
        async move {
        let s = format!("./assets/imgs/attrs/{}",alt);
        let src = nwsc::outfunc::out_thunb(src);
        let opt = nwsc::goresurl(dc, src.as_str(), s.as_str()).await;
        opt}}).buffered(20).collect::<Vec<bool>>().await;



        
    let mut offsets: Vec<u32> = iter.map(|i|i.unwrap()).collect::<Vec<u32>>();
    let ofs = &offsets[0..5];
    let url = ubapi.clone().mod_path("/精灵:").gourl();
    let mut selectormap =Vec::new();
    selectormap.push(Selectormap::new("img".into()));
    selectormap[0].add("#c4-tab1 > div > div.card-icon-placeholder > a".into());
    selectormap[0].add("#mw-content-text > div > div.row > div:nth-child(2) > div > div > div > div > div > div:nth-child(1) > table > tbody > tr > td > a".into());

    let results  = futures::stream::iter(ofs).map(|i|{
                let s = &i.to_string();
                let a =&ubc;
                let mut cul: String = url.clone().as_str().into();
                cul.push_str(s);
                let sm = selectormap.clone();

                async move {
                    let text = nwsc::ayasurl(&cul,a).await;
                    let mainout = task::res_ags(text,sm);
                    let out = mainout.iter().map(|ores|task::extract_attrs(ores.to_vec(), vec!["alt".into(),"src".into()])).collect::<Vec<Vec<Vec<String>>>>();
                out}
    }).buffered(20).collect::<Vec<Vec<Vec<Vec<String>>>>>().await;



    let optstr = serde_json::to_string(&results).unwrap();
    tokio::fs::write("opt.json", optstr).await.unwrap();

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




