use std::any::Any;

pub struct DataBase {
    pub key:String,
    pub value:Vec<dyn Any>
}