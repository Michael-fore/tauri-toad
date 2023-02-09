#[derive(Hash,Debug)]
pub struct Oracle {
    pub username:String,
    pub password:String,
    pub url:String,
    pub port:u32,
    pub database:String
}

impl Oracle{

    pub fn print(&self){
        println!("Oracle-{}-{}",self.username,self.database);
    }
    fn test_connection()->String{
        return "Success".to_string()
    }

    pub fn connect(&mut self){
        println!("{}",self.database);
    }

    fn execute(){
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