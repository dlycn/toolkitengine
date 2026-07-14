mod dbmg;
mod nwsc;
mod resp;

use dbmg::{Dbmbuilder as DMB, Table};
use nwsc::task;
use futures::stream::StreamExt;
use rusqlite::ToSql;


#[tokio::main]
async fn main(){
    let path = "./common/assetscrawler/assets.db";
    let mut dbm = DMB::connect(String::from(path)).unwrap();
    let data = task::meta_init();
    let syst = task::Tasksys::<task::Role>::from_data(data);
    let lim = 1000;
    let ubc = task::url_init().goclient();


    let offsets: Vec<usize> = (0..4972).step_by(lim).collect();

    let results: Vec<String> = futures::stream::iter(offsets)
        .map(|offset| {
            let mut dqp = syst.qp.clone();
            dqp.add_param("ID", lim, offset);
            let text = dqp.go();
            let refd =&ubc;
            async move {
                let out =task::bech_api_url(&text, refd).await;
                out
            }
        })
        .buffered(5)
        .collect::<Vec<_>>()
        .await;
    
    let resasys:Vec<Vec<String>> = results.iter()
    .map(|text|{task::api_ags(text.to_string())})
    .flatten().collect();

    let res: Vec<Vec<String>> = resasys.iter()
    .map(|txt| txt.iter().skip(1).cloned().collect())
    .collect();

    //println!("{:?}",res.iter().map(|i|{i.len()}).collect::<Vec<_>>());

    let mut ofs =0;

    let table = Table::new(syst.table.clone());
    let max = res.len();
    dbm.on(&table).pk("id").create();
    loop{
    let end = usize::min(ofs+lim+1,max); 
    
    let iter = res[ofs..end].iter().flat_map(
        |row| row.iter().map(
        |s| s as &dyn ToSql));
    let params = rusqlite::params_from_iter(iter);
    dbm.on(&table).append(end-ofs-1, params);
    if end == max{break}
    ofs+=lim
    }
    println!("name:{}", table.name);

}
