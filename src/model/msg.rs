// Result msg struct in response

use model::rut::Rut;
use model::item::{ Item, Collect };
use model::user::{ User, CheckUser };
use model::tag::{ Tag };

// general response msg struct
#[derive(Deserialize,Serialize,Debug)]
pub struct Msg {
    pub status: i32,
    pub message: String,
}

// msg for login or get user info
#[derive(Deserialize,Serialize,Debug)]
pub struct LoginMsg {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: CheckUser,
}

// result struct in response a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct RutMsg {
    pub status: i32,
    pub message: String,
    pub rut: Rut,
}

// result struct in response rut list
#[derive(Deserialize,Serialize,Debug)]
pub struct RutListMsg {
    pub status: i32,
    pub message: String,
    pub ruts: Vec<Rut>,
    pub count: usize,
}

// result struct in response an item 
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemMsg {
    pub status: i32,
    pub message: String,
    pub item: Item,
}

// result struct in response item list
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemListMsg {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub count: usize,
}

// result struct in response an items in a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct CollectMsg {
    pub status: i32,
    pub message: String,
    pub rut_id: String,
    pub collects: Vec<Collect>,
}

// result struct in response tag list
#[derive(Deserialize,Serialize,Debug)]
pub struct TagMsg {
    pub status: i32,
    pub message: String,
    pub tag: Tag,
}

// result struct in response tag list
#[derive(Deserialize,Serialize,Debug)]
pub struct TagListMsg {
    pub status: i32,
    pub message: String,
    pub tags: Vec<String>, // tag name
    pub count: usize,
}
