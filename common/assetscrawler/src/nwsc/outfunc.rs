pub fn out_thunb(s:String)->String{
    let v =s.split(".png/").collect::<Vec<&str>>();
    let n =v.iter().next().unwrap();
    let o =n.split("thumb/").collect::<Vec<&str>>();
    let mut p =o.concat();
    p.push_str(".png");
    p
}