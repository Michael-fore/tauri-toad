use mysql::{Pool, PooledConn};
use crate::drivers::return_types::{MyColumn,MyReturn,MyRow};

#[derive(Debug)]
pub struct MySQL {
    pub username:String,
    pub password:String,
    pub url:String,
    pub port:u32,
    pub database:String,
    pub pool: Option<Result<Pool,mysql::Error>>
}

impl MySQL{
    pub fn print(&self){
        println!("Mysql-{}-{}",self.username,self.database);
    }
    fn test_connection()->String{
        return "Success".to_string()
    }

    pub fn connect(&mut self){
        let conn_str = &*format!("mysql://{}:{}@{}/{}",self.username,self.password,self.url,self.database);
        self.pool =  match Pool::new(conn_str){
            Ok(pool)=>Some(Ok(pool)),
            Err(_)=>None
        };
    }

    fn get_conn(&self)->Result<PooledConn,mysql::Error>{
        return match self.pool{
            Some(pool)=>Ok(pool?.get_conn()?),
            None=>Err(mysql::Error::UrlError(mysql::UrlError::UnknownParameter("No connection pool is established".to_string())))
        };

    }

    fn execute(&self, sql: &str) -> MyReturn{
        let conn = self.get_conn();
        let vec = match conn.query(sql){
            Ok(x)=>x.to_owned().into_iter().collect(),
            Err(e)=> return Err(e)
        };
    }

    fn describe(){ 
    }

    fn commit(){ 
    }

    fn rollback(){ 
    }

    fn start_transaction(){
    }

    fn end_transaction(){
    }
}