
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyRow{
    pub values:Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyColumn{
    pub value:String,
    pub datatype:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyReturn {
    pub columns: Vec<MyColumn>,
    pub rows: Vec<MyRow>
}
