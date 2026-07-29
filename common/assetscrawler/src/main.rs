use assetscrawler::dbmg::{Dbmbuilder as DMB};
use assetscrawler::nwsc::{task,fix};
use assetscrawler::resp;
//use assetscrawler::nwsc;
#[tokio::main]
async fn main() {
    let path = "./assets/db/assets.db";
    let data = task::meta_init();
    let bugurl = task::url_init();
    let dbm: DMB = DMB::connect(String::from(path)).unwrap();
    let ubapi: &resp::UrlBuild =bugurl.api.get("seer.api").unwrap();
    let ubres: &resp::UrlBuild =bugurl.res.get("seer.h5.res").unwrap();
    
    let lim = 1000;
    let ofs = 0;
    let ubc: &reqwest::Client = &ubres.goclient();
    let syst = task::Tasksys::<task::PetSkin>::from_data(data);
    
    use futures::StreamExt;

    let mut stmt = dbm.manager
    .prepare("SELECT id FROM PetData").expect("id get wrong");
    let iter =stmt.query_map([], |row|{row.get::<usize,u32>(0)}).unwrap();
    //task::goconst(fix::Taskpie::GetAttrs).run(ubapi,ubc).await;

    // let offsets: Vec<u32> = iter.map(|i|i.unwrap()).collect::<Vec<u32>>();
    // let ofs = &offsets[0..5];
    // let url = ubapi.clone().mod_path("/精灵:").gourl();
    // let mut selectormap =Vec::new();
    // selectormap.push(task::Selectormap::new("img".into()));
    // selectormap[0].add("#c4-tab1 > div > div.card-icon-placeholder > a".into());
    // selectormap[0].add("#mw-content-text > div > div.row > div:nth-child(2) > div > div > div > div > div > div:nth-child(1) > table > tbody > tr > td > a".into());

    // let results  = futures::stream::iter(ofs).map(|i|{
    //             let s = &i.to_string();
    //             let a =ubc;
    //             let mut cul: String = url.clone().as_str().into();
    //             cul.push_str(s);
    //             let sm = selectormap.clone();

    //             async move {
    //                 let text = nwsc::ayasurl(&cul,a).await;
    //                 let mainout = task::res_ags(text,sm);
    //                 let out = mainout.iter().map(|ores|task::extract_attrs(ores.to_vec(), vec!["alt".into(),"src".into()])).collect::<Vec<Vec<Vec<String>>>>();
    //             out}
    // }).buffered(20).collect::<Vec<Vec<Vec<Vec<String>>>>>().await;



    // let optstr = serde_json::to_string(&results).unwrap();
    // tokio::fs::write("opt.json", optstr).await.unwrap();

    // syst.out_table_db(dbm, ubapi, ofs, lim).await;


    let mut stmt = dbm.manager
    .prepare("SELECT id FROM PetSkin").expect("id get wrong");
    let iter =stmt.query_map([], |row|{row.get::<usize,u32>(0)}).unwrap();
    let mut v =vec![];
    for i in iter{
        v.push(i.unwrap())
    }
    
    let mut p = ubres.clone();
    p.backup().use_path(2);
    println!("{}",p.gourl().as_str());

    task::go_dlink_resource(
        &p, v.split_off(0),
     "./assets/imgs/headers".to_string(), "png").await;
}




