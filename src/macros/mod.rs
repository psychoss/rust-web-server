
#[macro_export]
macro_rules! failed {
    ($r:expr,$c:expr) => {
    *($r).status_mut() = $c;
    match $r.start(){
        Ok(v)=>{
           let _= v.end();
            
        }
        _=>{}
    }
}
}

#[macro_export]
macro_rules! ok {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(_) => {
                return Err(());
            }
        }
    }
}
