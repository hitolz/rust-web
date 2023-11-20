use lazy_static::*;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{sync::Mutex, time::Duration};

lazy_static! {
    static ref MYSQL_POOL: Mutex<Vec<Pool<MySql>>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref RB: Mutex<RBatis> = Mutex::new(RBatis::new());
}

pub async fn init_db(database_url: String) {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .idle_timeout(Some(Duration::from_secs(30)))
        .connect(&database_url)
        .await
        .unwrap();

    let mut pools = MYSQL_POOL.lock().unwrap();
    (*pools).push(pool);

    let rbatis = RBatis::new();
    rbatis.init(MysqlDriver {}, &database_url).unwrap();
    let mut rb = RB.lock().unwrap();
    *rb = rbatis;

}

pub async fn get_mysql_pool() -> Pool<MySql> {
    let pools = MYSQL_POOL.lock().unwrap();
    unsafe { (*pools).get_unchecked(0).to_owned() }
}

pub async fn get_rb() -> RBatis {
    RB.lock().unwrap().clone()
}
