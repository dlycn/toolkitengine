mod dbmg;
mod nwsc;
mod resp;

use dbmg::{Dbmbuilder as DMB, tfm::Table};
use nwsc::task;
use rusqlite::ToSql;

#[tokio::main]
async fn main() {
    let path = "./common/assetscrawler/assets.db";
    let mut dbm = DMB::connect(String::from(path)).unwrap();
    let data = task::meta_init();
    let bugurl = task::url_init();
    let ub =bugurl.api.get("seer").unwrap();
    let lim = 1000;
    let mut ofs = 0;

    let syst = task::Tasksys::<task::Role>::from_data(data);
    let api = syst.go_table_url(ub,lim).await;

    let table = Table::new(syst.table.clone());
    let max = api.len();
    dbm.transaction(true).unwrap();
    dbm.on(&table).pk("id").create();
    loop {
        let end = usize::min(ofs + lim + 1, max);

        let iter = api[ofs..end]
            .iter()
            .flat_map(|row| row.iter().map(|s| s as &dyn ToSql));
        let params = rusqlite::params_from_iter(iter);
        dbm.on(&table).append(end - ofs - 1, params);
        if end == max {
            break;
        }
        ofs += lim
    }
    syst.apply_fix(&table, &mut dbm);
    dbm.transaction(false).unwrap();
}
