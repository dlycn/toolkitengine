use std::collections::HashMap;

use futures::StreamExt;
use crate::{resp,nwsc,task};

pub enum Taskpie{
    GetAttrs
}

pub struct TaskConst{
    pub init:Taskpie,
}
impl TaskConst{
    pub async fn run(self,ubs:&resp::UrlBuild,ubc:&reqwest::Client)->HashMap<String,bool>{
        match self.init {
            Taskpie::GetAttrs=>get_attrs(ubs,ubc).await,
        }
    }
}

pub async  fn get_attrs(ubapi:&resp::UrlBuild,ubc:&reqwest::Client)->HashMap<String,bool>{
    let url = ubapi.clone().use_path(2).gourl();
    let text = nwsc::ayasurl(url.as_str(),&ubc).await;
    let mut selectormap =vec![task::Selectormap::new("img".into())];
    selectormap[0].add("#myTabContent".into());
    let out = task::res_ags(text, selectormap);
    let opt = out.iter().map(|i|{task::extract_attrs(i.to_vec(), vec!["alt".into(),"src".into()])}).collect::<Vec<Vec<Vec<String>>>>();
    futures::stream::iter(opt[0].clone()).map(|d|{
        let (alt,src) =(d[0].clone(),d[1].clone());
        let dc =ubc;
        async move {
        let s = format!("./assets/imgs/attrs/{}",alt);
        let src = nwsc::outfunc::out_thunb(src);
        let opt = nwsc::goresurl(dc, src.as_str(), s.as_str()).await;
        (alt,opt)}}).buffered(20).collect::<HashMap<String,bool>>().await}