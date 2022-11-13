use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Error,Debug)]
enum ParseError{
    #[error("Input Tidak valid{0}")]
    InvalidId(#[from]std::num::ParseIntError),
    #[error("Input Tidak Valid{0}")]
    InvalidInput(&'static str),
    #[error("Missing Field: {0}")]
    MissingField(&'static str)
}

//CART STRUCTURE DATA
#[derive(Debug, Clone)]
struct Cart{
    id: i64,
    name: String,
    quantity: i64,
    outcome: i64
}

#[derive(Clone)]
struct Carts{
    carts: HashMap<i64, Cart>
}

impl Carts{
    fn new()->Self{
        Self { carts: HashMap::new()
        }
    }

    fn addCart(&mut self, carts: Cart){
        self.carts.insert(carts.id,carts); 
    }

    fn next_Id(&self)->i64{
        let mut nocrt: Vec<_> = self.carts.keys().collect();
        nocrt.sort();
        match nocrt.pop(){
            Some(id)=>id+1,
            None=>1
        }
    }

    fn into_vec(mut self)->Vec<Cart>{
        let mut Carts : Vec<_> = self.carts.drain()
        .map(|kv|kv.1)
        .collect();
        Carts.sort_by_key(|rec|rec.id);
        Carts
    }

}

fn parseCart(cart: &str)-> Result<Cart, ParseError>{
    let string: Vec<&str> = cart.split(',').collect();

    let id = match string.first(){
        Some(id) => id.parse::<i64>()?,
        None=>return Err(ParseError::InvalidInput("id - quantity"))
    };
    let name = match string.get(1).filter(|name|!name.is_empty()){
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField(("name")))
    };
    let quantity = match string.get(2){
        Some(quantity) => quantity.parse::<i64>()?,
        None=> return Err(ParseError::InvalidInput(("quantity")))
    };
    let outcome = match string.get(3){
        Some(price) => price.trim().parse::<i64>()?,
        None => return Err(ParseError::InvalidInput("outcome"))
    };

    Ok(Cart{id, name, quantity, outcome})
}

fn parseCarts(carts: String, verbose: bool) -> Carts{
    let mut crts = Carts::new();

    for(i, cart) in carts.split('\n').enumerate(){
        if !cart.is_empty(){
            match parseCart(cart){
                Ok(crt) => crts.addCart(crt),
                Err(err)=>if verbose { println!("System parse Error, {},{},{}", i+1, err,cart)} 
            }
        }
    }
    crts
}

fn loadCart(verbose: bool) -> std::io::Result<Carts>{
    let mut file=File::open(PathBuf::from("cart.csv"))?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parseCarts(buffer, verbose))
}

fn saveCart(carts: Carts) -> std::io::Result<()>{
    let mut file = OpenOptions::new().write(true).truncate(true)
    .open(PathBuf::from("cart.csv"))?;
    file.write_all(b"id,name,stock,outcome\n")?;
    file.flush()?;

    for cart in carts.into_vec().into_iter(){
        if cart.quantity.eq(&0){
            continue;
        }
        file.write_all(format!("{},{},{},{}\n",
        cart.id,
        cart.name,
        cart.quantity,
        cart.outcome).as_bytes())?;
    }
    Ok(())
}

//STORE STRUCTURE DATA
#[derive(Debug, Clone, PartialEq)]
struct Store{
    id: i64,
    name: String,
    quantity: i64,
    price: i64,
}

#[derive(Clone)]
struct Stores{
    hashmap: HashMap<i64, Store>
}

impl Stores{
    fn new()->Self{
        Self { hashmap: HashMap::new()
        }
    }

    fn addItem(&mut self, store: Store){
        self.hashmap.insert(store.id,store); 
    }

    fn into_vec(mut self)->Vec<Store>{
        let mut Stores : Vec<_> = self.hashmap.drain()
        .map(|kv|kv.1)
        .collect();

        Stores.sort_by_key(|rec|rec.id);
        Stores
    }

    fn sell(mut self, store: Store) ->(i64, i64){
        let temp: Vec<_> = self.hashmap.clone()
        .drain()
        .filter(|kv|kv.1.name==store.name)
        .collect();

        if temp.is_empty(){
            println!("Maaf barang tidak ditemukan");
            return(0,0)
        }else if store.quantity > temp.first().unwrap().1.quantity||temp.first()
        .unwrap().1.quantity == 0{
            println!("Maaf barang sudah sold out");
            return(0,0)
        };

        let newStore = parseStore(&format!("{},{},{},{}",
        temp.first().unwrap().1.id,
        temp.first().unwrap().1.name,
        temp.first().unwrap().1.quantity-store.quantity,
        temp.first().unwrap().1.price));  

        self.addItem(newStore.unwrap());

        (temp.first().unwrap().1.price * (store.quantity as i64), temp.first()
        .unwrap().1.quantity - store.quantity) 

    }
}

fn parseStore(store: &str)->Result<Store, ParseError>{
    let string: Vec<&str> = store.split(',').collect();

    let id = match string.first(){
        Some(id) => id.parse::<i64>()?,
        None=>return Err(ParseError::InvalidInput("id - quantity"))
    };

    let name = match string.get(1).filter(|name|!name.is_empty()){
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField(("name")))
    };

    let quantity = match string.get(2){
        Some(quantity) => quantity.parse::<i64>()?,
        None=> return Err(ParseError::InvalidInput(("quantity")))
    };

    let price = match string.get(3){
        Some(price) => price.trim().parse::<i64>()?,
        None => return Err(ParseError::InvalidInput("price"))
    };

    Ok( Store{id,name,quantity,price} )
}

fn parseStores(stores: String, verbose: bool) -> Stores{
    let mut strs = Stores::new();

    for(i, store) in stores.split('\n').enumerate(){
        if !store.is_empty(){
            match parseStore(store){
                Ok(str) => strs.addItem(str),
                Err(err)=>if verbose { println!("System parse Error, {},{},{}", i+1, err, store)}
            }
        }
    }
    strs
}

fn loadItem(verbose: bool) -> std::io::Result<Stores>{
    let mut file= File::open(PathBuf::from("store.csv"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parseStores(buffer, verbose))
}



fn saveItem(stores: Stores) -> std::io::Result<()>{
    let mut file = OpenOptions::new().write(true).truncate(true)
    .open(PathBuf::from("store.csv"))?;
    file.write_all(b"id,name,stock,price\n")?;
    file.flush()?;

    for store in stores.into_vec().into_iter(){
        if store.quantity.eq(&0){
            continue;
        }
        file.write_all(format!("{},{},{},{}\n",
        store.id,
        store.name,
        store.quantity,
        store.price).as_bytes())?;
    }
    Ok(())
}

#[derive(StructOpt)]
enum Command{
    List,
    inCart,
    Checkout,
    Cart{
        id: i64,
        quantity: i64
    }
}

#[derive(StructOpt)]
#[structopt(about="Rust E-Commerce")]
struct Opt{
#[structopt(subcommand)]
cmd: Command,
#[structopt(short, help = "verbose")]
verbose: bool
}

impl Opt{
    fn run(opt: Opt) -> Result<(), std::io::Error>{
        match opt.cmd{
            Command::List=>{
                let stores = loadItem(opt.verbose)?;
                for store in stores.into_vec(){
                    println!("{:?}", store)
                }
                 Ok(())
            }

            Command::inCart=>{
                let Carts = loadCart(opt.verbose)?;
                for Cart in Carts.into_vec(){
                    println!("{:?}", Cart)
                }
                 Ok(())
            }

            Command::Cart { id, quantity }=>{
                let stores = loadItem(opt.verbose)?;
                let mut carts= loadCart(opt.verbose)?;
                
                    match stores.hashmap.get(&id){
                        Some(x)=>{
                            if(x.quantity>=quantity){
                            let c = Cart{
                                id: x.id,
                                name: x.name.to_string(),
                                quantity: quantity,
                                outcome: x.price * quantity
                            };  
                            println!("Total Price: Rp.{}", c.outcome);
                            carts.addCart(c);
                            }
                        },
                        None => {}  
                          
                    }
                
                    saveCart(carts)?;
                    Ok(())
                
            }

            Command::Checkout{}=>{
                let stores = loadItem(opt.verbose)?;
                let carts= loadCart(opt.verbose)?;

                let mut newStores = stores.clone();
                let mut total:i64 = 0;
                for cart in carts.into_vec().into_iter(){
                    match stores.hashmap.get(&cart.id){
                        Some(x)=>{
                            if(x.quantity >= cart.quantity){
                                let newStore = Store{
                                name: x.name.to_string(),
                                id: x.id,
                                price :x.price,
                                quantity: x.quantity - cart.quantity,
                                }; 
                                total += cart.outcome;
                            newStores.addItem(newStore)
                            };
                        }, 
                        None=> {}
                    }  
                }
            println!("Total Belanja: Rp.{}",total);   
            saveItem(newStores)?;
            let newcarts= Carts::new();
            saveCart(newcarts)?;
            Ok(())
            }
        }
    }
}

fn main() {
    let arguments = Opt::from_args();
    if let Err(e) = Opt::run(arguments) {
        println!("Error: {}", e);
    }
}   