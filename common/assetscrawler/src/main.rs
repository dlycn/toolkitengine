mod dbmg;
mod nwsc;
mod resp;

use dbmg::{Dbmbuilder as DMB, tfm::Table};
use nwsc::task;
use rusqlite::ToSql;

#[tokio::main]
async fn main() {
    let path = "./assets/db/assets.db";
    let mut dbm = DMB::connect(String::from(path)).unwrap();
    let data = task::meta_init();
    let bugurl = task::url_init();
    let ubapi =bugurl.api.get("seer.api").unwrap();
    let ubres =bugurl.res.get("seer.flash.res").unwrap();
    let lim = 1000;
    let mut ofs = 0;

    let ubc = ubres.goclient();




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





    let syst = task::Tasksys::<task::PetSkin>::from_data(data);
    let api = syst.go_table_url(ubapi,lim).await;

    let table = Table::new(syst.table.clone());
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
    syst.apply_fix(&table, &mut dbm);
    dbm.transaction(false).unwrap();
}
