
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyRow{
    values:Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyColumn{
    value:String,
    datatype:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyReturn {
    columns: Vec<MyColumn>,
    rows: Vec<MyRow>
}
