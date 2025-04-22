/*use bson::{doc, Bson, Document};
use log::warn;
use mongodb::{sync};
use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Collection, Database};
use crate::query::ReqItem;

pub struct SyncDB {
    client: Client,
    db: Database,
    reqitems_collection: Collection<ReqItem>,
}
impl SyncDB {
    pub fn new() -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").run().unwrap();
        let client: Client = Client::with_options(client_options).unwrap();
        let db: Database = client.database("local");
        let reqitems_collection: Collection<ReqItem> = db.collection::<ReqItem>("items2");
        Self {
            client: client,
            db: db,
            reqitems_collection: reqitems_collection,
        }
    }
    pub fn q_targets(&self) -> Vec<String> {
        let res: Vec<Bson> = self.reqitems_collection.distinct("target", Document::default()).run().unwrap();
        let mut targets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            warn!("b: {:?}", b);
            targets.push(b.as_str().unwrap().to_string());
        });
        targets
    }
    pub fn q_imo_by_target(&self, target: &str) -> Vec<String> {
        let filter = doc! { "target":target };
        let res: Vec<Bson> = self.reqitems_collection.distinct("imo", filter).run().unwrap();
        let mut rets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            rets.push(b.as_str().unwrap().to_string());
        });
        warn!("b: {:?}", rets);
        rets
    }
    pub fn catalogs(&self, target: &str) -> Vec<(String, String)> {
        let mut ret:Vec<(String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target}},
            doc! {"$group": {"_id": { "factory_name": "$factory_name", "eq_type": "$eq_type"}}},
            doc! {"$project": {"_id":0,  "factory_name": "$_id.factory_name", "eq_type": "$_id.eq_type"}},
        ];
        
       self.reqitems_collection.aggregate(pipeline).run().unwrap().for_each(|r| {
           let mut factory_name=String::new();
           let mut eq_type=String::new();
          match  r{
              Ok(doc) => {
                match  doc.get("factory_name") {
                    None => {}
                    Some(f) => {factory_name+=f.as_str().unwrap();}
                };
                  match  doc.get("eq_type") {
                      None => {}
                      Some(f) => {eq_type+=f.as_str().unwrap();}
                  };
              }
              Err(_) => {}
          }
           if(!factory_name.is_empty() && !eq_type.is_empty()) {
               ret.push((factory_name,eq_type));
            }
          
        });
        //ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }
    
    pub fn parts_by_catalog(&self, factory_name: &str, eq_type: &str) -> Vec<(String, String)> {
        let mut ret:Vec<(String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"factory_name": factory_name,"eq_type": eq_type}},
            doc! {"$group": {"_id": { "part_id": "$part_id", "part_descr": "$part_descr"}}},
            doc! {"$project": {"_id":0,  "part_id": "$_id.part_id", "part_descr": "$_id.part_descr"}},
            doc! {"$sort": {"part_id": 1,"part_descr":1}},
        ];

        self.reqitems_collection.aggregate(pipeline).run().unwrap().for_each(|r| {
            let mut factory_name=String::new();
            let mut eq_type=String::new();
            match  r{
                Ok(doc) => {
                    match  doc.get("part_id") {
                        None => {}
                        Some(f) => {factory_name+=f.as_str().unwrap();}
                    };
                    match  doc.get("part_descr") {
                        None => {}
                        Some(f) => {eq_type+=f.as_str().unwrap();}
                    };
                }
                Err(_) => {}
            }
            if(!factory_name.is_empty() && !eq_type.is_empty()) {
                ret.push((factory_name,eq_type));
            }

        });
        ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }
    pub fn parts_by_factory_name(&self, factory_name: &str) -> Vec<(String, String, String)> {
        let mut ret:Vec<(String,String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"factory_name": factory_name}},
            doc! {"$group": {"_id": { "eq_type": "$eq_type", "part_id": "$part_id", "part_descr": "$part_descr"}}},
            doc! {"$project": {"_id":0,"eq_type": "$_id.eq_type",  "part_id": "$_id.part_id", "part_descr": "$_id.part_descr"}},
            doc! {"$sort": {"eq_type": 1,"part_id": 1,"part_descr":1}},
        ];

        self.reqitems_collection.aggregate(pipeline).run().unwrap().for_each(|r| {
            let mut a=String::new();
            let mut b=String::new();
            let mut c=String::new();
            match  r{
                Ok(doc) => {
                    match  doc.get("eq_type") {
                        None => {}
                        Some(f) => {a+=f.as_str().unwrap();}
                    };
                    match  doc.get("part_id") {
                        None => {}
                        Some(f) => {b+=f.as_str().unwrap();}
                    };
                    match  doc.get("part_descr") {
                        None => {}
                        Some(f) => {c+=f.as_str().unwrap();}
                    };
                }
                Err(_) => {}
            }
            if(!a.is_empty() && !b.is_empty()&& !c.is_empty()) {
                ret.push((a,b,c));
            }

        });
        ret.iter().for_each(|(a,b,c)| { println!("{} {} {}", a, b,c); });
        ret
    }
    pub fn parts_by_target(&self, target: &str) -> Vec<(String, String, String,String)> {
        let mut ret:Vec<(String,String,String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target}},
            doc! {"$group": {"_id": { "factory_name": "$factory_name","eq_type": "$eq_type", "part_id": "$part_id", "part_descr": "$part_descr"}}},
            doc! {"$project": {"_id":0,"factory_name": "$_id.factory_name","eq_type": "$_id.eq_type",  "part_id": "$_id.part_id", "part_descr": "$_id.part_descr"}},
            doc! {"$sort": {"factory_name": 1,"eq_type": 1,"part_id": 1,"part_descr":1}},
        ];

        self.reqitems_collection.aggregate(pipeline).run().unwrap().for_each(|r| {
            let mut a=String::new();
            let mut b=String::new();
            let mut c=String::new();
            let mut d=String::new();
            match  r{
                Ok(doc) => {
                    match  doc.get("factory_name") {
                        None => {}
                        Some(f) => {a+=f.as_str().unwrap();}
                    };
                    match  doc.get("eq_type") {
                        None => {}
                        Some(f) => {b+=f.as_str().unwrap();}
                    };
                    match  doc.get("part_id") {
                        None => {}
                        Some(f) => {c+=f.as_str().unwrap();}
                    };
                    match  doc.get("part_descr") {
                        None => {}
                        Some(f) => {d+=f.as_str().unwrap();}
                    };
                }
                Err(_) => {}
            }
            if(!a.is_empty() && !b.is_empty()&& !c.is_empty()&& !d.is_empty()) {
                ret.push((a,b,c,d));
            }

        });
        ret.iter().for_each(|(a,b,c,d)| { println!("{} {} {}", a, b,c); });
        ret
    }
    
}*/