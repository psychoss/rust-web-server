
#[macro_export]
macro_rules! failed {
    ($r:expr,$c:expr) => {
    *($r).status_mut() = $c;
    match $r.start(){
        Ok(v)=>{
            v.end();
            println!("end");
        }
        _=>{}
    }
}
}
