

use bson::{doc, Bson, Document};
use log::warn;
use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use crate::query::{ReqItem, UNKNOWN_ITEM};

#[derive(Clone)]
pub struct AsyncDB {
    client: Client,
    db: Database,
    reqitems_collection: Collection<ReqItem>,
}
impl AsyncDB {
    pub async fn new() -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

        let client:Client = Client::with_options(client_options).unwrap();
        let db = client.database("local");
        let reqitems_collection: Collection<ReqItem> = db.collection::<ReqItem>("items2");
        Self {
            client: client,
            db: db,
            reqitems_collection: reqitems_collection,
        }
    }
    
    pub async fn q_targets(&self) -> Vec<String> {
        let res: Vec<Bson> = self.reqitems_collection.distinct("target", Document::default()).await.unwrap();
        let mut targets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            warn!("b: {:?}", b);
            targets.push(b.as_str().unwrap().to_string());
        });
        targets
    }

    pub async fn q_clients(&self) -> Vec<String> {
        let res: Vec<Bson> = self.reqitems_collection.distinct("client_name", Document::default()).await.unwrap();
        let mut targets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            warn!("b: {:?}", b);
            targets.push(b.as_str().unwrap().to_string());
        });
        targets
    }

    pub async fn q_imo_by_target(&self, target: &str) -> Vec<String> {
        let filter = doc! { "target":target };
        let res: Vec<Bson> = self.reqitems_collection.distinct("imo", filter).await.unwrap();
        let mut rets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            rets.push(b.as_str().unwrap().to_string());
        });
        rets
    }

    pub async fn catalogs(&self, target: &str) -> Vec<(String, String)> {
        let mut ret:Vec<(String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target}},
            doc! {"$group": {"_id": { "factory_name": "$factory_name", "eq_type": "$eq_type"}}},
            doc! {"$project": {"_id":0,  "factory_name": "$_id.factory_name", "eq_type": "$_id.eq_type"}},
        ];
        let mut results=self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            //let mut factory_name=UNKNOWN_ITEM.clone().to_owned();
            //let mut eq_type=UNKNOWN_ITEM.clone().to_owned();
            let mut factory_name="".to_owned();
            let mut eq_type="".to_owned();;
            match result {
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
        }
        //ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }

    pub async fn parts_by_catalog(&self, factory_name: &str, eq_type: &str) -> Vec<(String, String)> {
        let mut ret:Vec<(String,String)>=vec![];
        let pipeline = vec![
            doc! {"$match": {"factory_name": factory_name,"eq_type": eq_type}},
            doc! {"$group": {"_id": { "part_id": "$part_id", "part_descr": "$part_descr"}}},
            doc! {"$project": {"_id":0,  "part_id": "$_id.part_id", "part_descr": "$_id.part_descr"}},
            doc! {"$sort": {"part_id": 1,"part_descr":1}},
        ];
        let mut results=self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            let mut factory_name="".to_owned();
            let mut eq_type="".to_owned();
            match result {
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
        }
        //ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }

    pub async fn serials_by_target_catalog(&self,target: &str, factory_name: &str, eq_type: &str) -> Vec<String> {
        let mut ret:Vec<String>=vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target,"factory_name": factory_name,"eq_type": eq_type}},
            doc! {"$unwind": "$serials"},
            doc! {"$group": {"_id": { "serials": "$serials"}}},
            doc! {"$project": {"_id":0,  "serials": "$_id.serials"}},
            doc! {"$sort": {"serials": 1}},
        ];
        let mut results=self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            let mut serials="".to_owned();
         
            match result {
                Ok(doc) => {
                    match  doc.get("serials") {
                        None => {}
                        Some(f) => {serials+=f.as_str().unwrap();}
                    };
                }
                Err(_) => {}
            }
            if(!serials.is_empty()) {
                ret.push((serials));
            }
        }
        ret.iter().for_each(|(a)| { println!("{}", a); });
        ret
    }

    pub async fn q_place_by_client_target(&self,client_name: &str, target: &str) -> Vec<String> {
        let filter = doc! { "target":target,"client_name":client_name };
        let res: Vec<Bson> = self.reqitems_collection.distinct("target_place", filter).await.unwrap();
        let mut rets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            rets.push(b.as_str().unwrap().to_string());
        });
        rets
    }
    
}