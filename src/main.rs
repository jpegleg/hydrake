extern crate redis;
use redis::Commands;
use chrono::prelude::*;
use std::{thread, time};

fn hydrater() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _ : () = con.set("product_list", "static_assets_be,dynamic_assets_be,escrow_assets_fe,escrow_assets_be,holdings,custom,nethost")?;
    let _ : () = con.set("static_assets_be", "PC-0000-0001_12ef34f94ec24d2ee4ea902d0d60fd99")?;
    let _ : () = con.set("dynamic_assets_be", "PC-0000-0002_8b313cd4f8c661664bd61984b8b4161c")?;
    let _ : () = con.set("escrow_assets_fe", "PC-0005-0003_2fe08d4cee3d38e607eb3bf4eb4b88cd")?;
    let _ : () = con.set("escrow_assets_be", "PC-0003-0004_b0785b205ed3fa7c3e27aa88a3551423")?;
    let _ : () = con.set("holdings", "PC-0005-0005_2cc86f84c691a7fbefa25cc43457e907")?;
    let _ : () = con.set("custom", "PC-0002-0006_50881370f1c4e9a5a75c06b9660457af")?;
    let _ : () = con.set("nethost", "PC-0001-0007_019542a4104b4daf72f163e410a4d4b7")?;
    let _ : () = con.set("privacy_policy", "UFJJVkFDWSBQT0xJQ1kKCldlIGRvIG5vdCBzdG9yZSBvciB1c2UgYW55IFBJSSBkYXRhLgoKVGhlIG9ubHkgZGF0YSB3ZSBjb2xsZWN0IGlzIHRoZSBkYXRhCnByb3ZpZGVkIGJ5IHlvdXIgY2xpZW50IHJlcXVlc3RzLgoKVGhpcyBkYXRhIG1heSBpbmNsdWRlIElQIGFkZHJlc3MKb3Igb3RoZXIgYnJvd3NlciBpbmZvcm1hdGlvbiB0aGF0CmlzIHNlbnQgdG8gb3VyIHNlcnZlcnMuIFdlIGRvIG5vCnNoYXJlIHRoYXQgZGF0YSB3aXRoIGFueW9uZS4KCgo=")?;

    Ok(())
}

fn main() {
    loop {
        let hydration = hydrater();
        let initc = Utc::now();
        let _conn = match hydration  {
            Ok(()) => (),
            Err(error) => panic!("{} Problem connecting to redis: {:?}", initc, error),
        };

        let millis = time::Duration::from_millis(100000);
        thread::sleep(millis);
    }
}
