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

    fn execute(&self, sql: &str) -> MyReturn {
        let conn = match self.get_conn(){
            Ok(conn)=>conn,
            Err(err)=> return MyReturn{
                columns:vec![MyColumn{value:e.to_string(),datatype:"ERROR".to_string()}],
                rows:vec![]
            }
        };

        let vec = match conn.query(sql){
            Ok(x)=>x.to_owned().into_iter().collect(),
            Err(e)=>  return MyReturn{
                columns:vec![MyColumn{value:e.to_string(),datatype:"ERROR".to_string()}],
                rows:vec![]
            }
        };

        if vec.is_empty(){
            return MyReturn{columns:vec![],rows:vec![]};
        }

        let mut out_columns = Vec::new();
        for i in vec[0].columns_ref().clone(){
            // let columns = i.; 
            out_columns.push(MyColumn{value:i.name_str().to_string(), datatype:"".to_string()})
        }
    
        for i in vec {
    
            //as of now every row loops through and filles the columns
            // as a side project i am gonna leave it, but will proly 
            //chagne to {key:value} output style
    
            let vals = i.unwrap();
           
            let mut vals_vec = Vec::new();
            for val in vals{   
                vals_vec.push(val.as_sql(true).to_owned())
                // r.push(val.as_sql(true).to_owned());
            }
            r.push(MyRow{values:vals_vec});
        }
        let output = MyReturn{columns:out_columns, rows:r}; 
        return output;

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