use reqwest::*;

// resource_personal
#[derive(Clone,Debug)]
pub struct UrlBuild{
    pub main_path:String,
    pub child_path:Vec<String>,
    pub headers:Option<header::HeaderMap>
}
impl UrlBuild{
    pub fn new(url:String)->Self{
        Self { main_path: url.clone(), child_path: vec![url],headers:None }
    }

    pub fn add_headers(&mut self,headers:header::HeaderMap) ->&mut Self {
        self.headers = Some(headers);
        self}

    pub fn add_path(&mut self,urlc:String)->&mut Self {
        self.child_path.push(urlc);
        self
    }


    ///- jsut like X.mod_path("/xxx")
    pub fn mod_path(&mut self,curl:&str)->&mut Self {
        self.main_path.push_str(curl);
        self
    }

    pub fn use_path(&mut self,num:usize)->&mut Self {
        let n = self.child_path.len();
        if num == 0{self.backup(num)} else {
            if n+1>=num{
                self.main_path.push_str(self.child_path[num].as_str())}
        self}
    }
    pub fn backup(&mut self,num:usize)->&mut Self {
        self.main_path=self.child_path[num].clone();
        self
    }

    pub fn gourl(&mut self)->reqwest::Url{
        reqwest::Url::parse(&self.main_path).unwrap()
    }

    pub fn goclient(&self)->Client{
        let mut clientbuilder = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3 Edg/150.0.0.0")
        .timeout(std::time::Duration::from_secs(64));
        let mut headers = if 
        let Some(h) = &self.headers {
            h.clone()
        } else {
            header::HeaderMap::new()
        };
        headers.append("Referer", self.main_path.parse().unwrap());
        headers.append("connection","keep-alive".to_string().parse().unwrap());
        clientbuilder = clientbuilder.default_headers(headers.clone());
        clientbuilder.build().unwrap()
    }

    }


