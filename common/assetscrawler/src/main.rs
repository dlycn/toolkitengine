mod dbmg;
mod nwsc;
mod resp;

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
    
    syst.out_table_db(dbm, ubapi, ofs, lim).await;


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




