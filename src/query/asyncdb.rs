use crate::query::{ReqItem, UNKNOWN_ITEM};
use bson::{Bson, Document, doc};
use log::warn;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Cursor, Database};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use uuid::Uuid;

#[derive(Clone)]
pub struct AsyncDB {
    client: Client,
    db: Database,
    reqitems_collection: Collection<ReqItem>,
    active_zip_requests_collection: Collection<ReqItem>,
}
impl AsyncDB {
    pub async fn new() -> Self {
        let cs="mongodb://localhost:27017";
/*        
https://ruvds.com/ru/helpcenter/kak-ustanovit-mongodb-na-ubuntu/
https://ruvds.com/ru/helpcenter/mongodb-security-ubuntu-20-04/

db.createUser(
            {
                user: "superadmin",
                pwd: "ssww@@31",
                roles: [
                "userAdminAnyDatabase",
                "dbAdminAnyDatabase",
                "readWriteAnyDatabase"
                ]
            }
        )*/

        //let cs="mongodb://superadmin:ssww%40%4031@185.91.52.4:27017/?retryWrites=true&loadBalanced=false&serverSelectionTimeoutMS=5000&connectTimeoutMS=10000&authSource=admin&authMechanism=SCRAM-SHA-256";
        
        let client_options = ClientOptions::parse(cs)
            .await
            .unwrap();

        let client: Client = Client::with_options(client_options).unwrap();
        let db = client.database("local");
        let reqitems_collection: Collection<ReqItem> = db.collection::<ReqItem>("items2");
        let active_zip_requests_collection: Collection<ReqItem> =
            db.collection::<ReqItem>("active_zip_requests");
        Self {
            client: client,
            db: db,
            reqitems_collection: reqitems_collection,
            active_zip_requests_collection: active_zip_requests_collection,
        }
    }

    pub async fn q_targets(&self) -> Vec<String> {
        let res: Vec<Bson> = self
            .reqitems_collection
            .distinct("target", Document::default())
            .await
            .unwrap();
        let mut targets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            targets.push(b.as_str().unwrap().to_string());
        });
        targets
    }

    pub async fn q_clients(&self) -> Vec<String> {
        let res: Vec<Bson> = self
            .reqitems_collection
            .distinct("client_name", Document::default())
            .await
            .unwrap();
        let mut targets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            targets.push(b.as_str().unwrap().to_string());
        });
        targets
    }

    pub async fn q_imo_by_target(&self, target: &str) -> Vec<String> {
        let filter = doc! { "target":target };
        let res: Vec<Bson> = self
            .reqitems_collection
            .distinct("imo", filter)
            .await
            .unwrap();
        let mut rets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            rets.push(b.as_str().unwrap().to_string());
        });
        rets
    }

    pub async fn catalogs(&self, target: &str) -> Vec<(String, String)> {
        let mut ret: Vec<(String, String)> = vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target}},
            doc! {"$group": {"_id": { "factory_name": "$factory_name", "eq_type": "$eq_type"}}},
            doc! {"$project": {"_id":0,  "factory_name": "$_id.factory_name", "eq_type": "$_id.eq_type"}},
        ];
        let mut results = self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            //let mut factory_name=UNKNOWN_ITEM.clone().to_owned();
            //let mut eq_type=UNKNOWN_ITEM.clone().to_owned();
            let mut factory_name = "".to_owned();
            let mut eq_type = "".to_owned();
            match result {
                Ok(doc) => {
                    match doc.get("factory_name") {
                        None => {}
                        Some(f) => {
                            factory_name += f.as_str().unwrap();
                        }
                    };
                    match doc.get("eq_type") {
                        None => {}
                        Some(f) => {
                            eq_type += f.as_str().unwrap();
                        }
                    };
                }
                Err(_) => {}
            }
            if (!factory_name.is_empty() && !eq_type.is_empty()) {
                ret.push((factory_name, eq_type));
            }
        }
        //ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }

    pub async fn parts_by_catalog(
        &self,
        factory_name: &str,
        eq_type: &str,
    ) -> Vec<(String, String)> {
        let mut ret: Vec<(String, String)> = vec![];
        let pipeline = vec![
            doc! {"$match": {"factory_name": factory_name,"eq_type": eq_type}},
            doc! {"$group": {"_id": { "part_id": "$part_id", "part_descr": "$part_descr"}}},
            doc! {"$project": {"_id":0,  "part_id": "$_id.part_id", "part_descr": "$_id.part_descr"}},
            doc! {"$sort": {"part_id": 1,"part_descr":1}},
        ];
        let mut results = self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            let mut factory_name = "".to_owned();
            let mut eq_type = "".to_owned();
            match result {
                Ok(doc) => {
                    match doc.get("part_id") {
                        None => {}
                        Some(f) => {
                            factory_name += f.as_str().unwrap();
                        }
                    };
                    match doc.get("part_descr") {
                        None => {}
                        Some(f) => {
                            eq_type += f.as_str().unwrap();
                        }
                    };
                }
                Err(_) => {}
            }
            if (!factory_name.is_empty() && !eq_type.is_empty()) {
                ret.push((factory_name, eq_type));
            }
        }
        //ret.iter().for_each(|(a,b)| { println!("{} {}", a, b); });
        ret
    }

    pub async fn serials_by_target_catalog(
        &self,
        target: &str,
        factory_name: &str,
        eq_type: &str,
    ) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        let pipeline = vec![
            doc! {"$match": {"target": target,"factory_name": factory_name,"eq_type": eq_type}},
            doc! {"$unwind": "$serials"},
            doc! {"$group": {"_id": { "serials": "$serials"}}},
            doc! {"$project": {"_id":0,  "serials": "$_id.serials"}},
            doc! {"$sort": {"serials": 1}},
        ];
        let mut results = self.reqitems_collection.aggregate(pipeline).await.unwrap();
        while let Some(result) = results.next().await {
            let mut serials = "".to_owned();

            match result {
                Ok(doc) => {
                    match doc.get("serials") {
                        None => {}
                        Some(f) => {
                            serials += f.as_str().unwrap();
                        }
                    };
                }
                Err(_) => {}
            }
            if (!serials.is_empty()) {
                ret.push((serials));
            }
        }
        //ret.iter().for_each(|(a)| { println!("{}", a); });
        ret
    }

    pub async fn q_place_by_client_target(&self, client_name: &str, target: &str) -> Vec<String> {
        let filter = doc! { "target":target,"client_name":client_name };
        let res: Vec<Bson> = self
            .reqitems_collection
            .distinct("target_place", filter)
            .await
            .unwrap();
        let mut rets: Vec<String> = Vec::new();
        res.iter().for_each(|b| {
            rets.push(b.as_str().unwrap().to_string());
        });
        rets
    }

    pub async fn q_add_drafts(&self, drafts: Vec<ReqItem>) {
        let mut dra = drafts.clone();
        dra.iter_mut().for_each(|d| {
            d.id = Uuid::new_v4().to_string();
        });

        let ret = self.active_zip_requests_collection.insert_many(dra).await;
        match ret {
            Ok(k) => {}
            Err(e) => {
                warn!("q_add_drafts: {:?}", e);
            }
        }
    }

    pub async fn q_requests_by_status(&self, status: &i32) -> Vec<(String, String, String, i32)> {
        let pipeline = vec![
            doc! {"$match": {"status": status}},
            doc! {"$group": { "_id": {"request_day" :  "$request_day","request_month" : "$request_month","request_year" : "$request_year", "client_name": "$client_name","request_id": "$request_id"},"count": { "$sum": 1 } } },
            doc! {"$project":{"_id":0, "request_id": "$_id.request_id" , "request_day": "$_id.request_day", "request_month": "$_id.request_month", "request_year": "$_id.request_year", "client_name": "$_id.client_name","count": "$count"}},
            doc! {"$sort": {"id": 1}},
        ];

        let mut results = self
            .active_zip_requests_collection
            .aggregate(pipeline)
            .await
            .unwrap();
        let mut ret: Vec<(String, String, String, i32)> = vec![];
        while let Some(result) = results.next().await {
            let mut id: String = "".to_owned();
            let mut client_name: String = "".to_owned();
            let mut request_id: String = "".to_owned();
            let mut count: i32 = 0;
            match result {
                Ok(doc) => {
                    match doc.get("request_day") {
                        None => {}
                        Some(d) => {
                            id = d.as_i32().unwrap_or(0).to_string();
                        }
                    }
                    match doc.get("request_month") {
                        None => {}
                        Some(d) => {
                            id = id + " " + d.as_i32().unwrap_or(0).to_string().as_str();
                        }
                    }
                    match doc.get("request_year") {
                        None => {}
                        Some(d) => {
                            id = id + " " + d.as_i32().unwrap_or(0).to_string().as_str();
                        }
                    }
                    match doc.get("client_name") {
                        None => {}
                        Some(d) => {
                            client_name = d.as_str().unwrap_or("").to_owned();
                        }
                    }
                    match doc.get("request_id") {
                        None => {}
                        Some(d) => {
                            request_id = d.as_str().unwrap_or("").to_owned();
                        }
                    }
                    match doc.get("count") {
                        None => {}
                        Some(d) => {
                            count = d.as_i32().unwrap_or(0);
                        }
                    }
                }
                Err(e) => {}
            }

            ret.push((id, client_name, request_id, count));
        }
        ret
    }
    pub async fn q_requests_by_orderid(&self, orderid: String) -> Vec<ReqItem> {
        let mut results: Cursor<ReqItem> = self
            .active_zip_requests_collection
            .find(doc! {"request_id" : orderid})
            .sort(doc! {"part_no": 1})
            .await
            .unwrap();
        //let mut results: Cursor<ReqItem> =self.active_zip_requests_collection.find(doc! {"request_id" : orderid}).await.unwrap();
        let mut ret: Vec<ReqItem> = vec![];
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    ret.push(doc);
                }
                Err(e) => {}
            }
        }
        ret
    }
    pub async fn q_requests_by_orderid_status(&self, orderid: String, status: i32) -> Vec<ReqItem> {
        let mut results: Cursor<ReqItem> = self
            .active_zip_requests_collection
            .find(doc! {"request_id" : orderid,"status" : status})
            .sort(doc! {"part_no": 1})
            .await
            .unwrap();
        //let mut results: Cursor<ReqItem> =self.active_zip_requests_collection.find(doc! {"request_id" : orderid}).await.unwrap();
        let mut ret: Vec<ReqItem> = vec![];
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    ret.push(doc);
                }
                Err(e) => {}
            }
        }
        ret
    }


    pub async fn q_requests_suplyers_orderid(&self, orderid: String) -> Vec<String> {
        let pipeline = vec![
            doc!  {"$match": { "status": 80, "request_id":orderid } },
            doc! {"$group": { "_id": {"supplier_person" :  "$supplier_person" } }},
            doc! {"$project":{"_id":0, "supplier_person": "$_id.supplier_person" }},
            doc! {"$sort": {"supplier_person": 1}},
        ];

        let mut results = self
            .active_zip_requests_collection
            .aggregate(pipeline)
            .await
            .unwrap();
        let mut ret: Vec<String> = vec![];
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    match doc.get("supplier_person") {
                        None => {}
                        Some(d) => {
                           ret.push(d.as_str().unwrap_or("").to_owned());
                        }
                    }
                }
                Err(e) => {}
            }
        }
        ret
    }
    pub async fn q_update_status(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            match p.status {
                10 => {
                    let update = doc! { "$set": doc! {"status": 20} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                20 => {
                    let update = doc! { "$set": doc! {"status": 30} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                30 => {
                    let update = doc! { "$set": doc! {"status": 40} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                40 => {
                    let update = doc! { "$set": doc! {"status": 50} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                50 => {
                    let update = doc! { "$set": doc! {"status": 60} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                60 => {
                    let update = doc! { "$set": doc! {"status": 70} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                70 => {
                    let update = doc! { "$set": doc! {"status": 80} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                80 => {
                    let update = doc! { "$set": doc! {"status": 90} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                90 => {
                    let update = doc! { "$set": doc! {"status": 100} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                100 => {
                    let update = doc! { "$set": doc! {"status": 110} };
                    let res = self
                        .active_zip_requests_collection
                        .update_one(filter, update)
                        .await;
                }
                110 => {}
                _ => {}
            }
        }
    }
    pub async fn q_update_supplyer(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let supp = p.supplier_person.clone();
            if (!supp.is_empty()) {
                let filter = doc! { "id": p.id.clone() };
                let update = doc! { "$set": doc! {"status": 30,"supplier_person": supp} };
                let res = self
                    .active_zip_requests_collection
                    .update_one(filter, update)
                    .await;
            }
        }
    }
    pub async fn q_update_supplyer_price(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let supp_price = p.part_price.clone();
            let supp_delivery = p.part_delivery.clone();
            if (supp_price > 0.0 && supp_delivery > 0) {
                let filter = doc! { "id": p.id.clone() };
                let update = doc! { "$set": doc! {"status": 40,"part_price": supp_price,"part_delivery": supp_delivery} };
                let res = self
                    .active_zip_requests_collection
                    .update_one(filter, update)
                    .await;
            }
        }
    }
    pub async fn q_approve_price(&self, parts: Vec<ReqItem>, price: f32) {
        for p in parts.iter() {
            let summ_price = price;
            if (summ_price > 0.0) {
                let filter = doc! { "id": p.id.clone() };
                let update = doc! { "$set": doc! {"status": 50,"part_price_sum": summ_price} };
                let res = self
                    .active_zip_requests_collection
                    .update_one(filter, update)
                    .await;
            }
        }
    }
    pub async fn q_accept_price(&self, parts: Vec<ReqItem>, price: f32) {
        for p in parts.iter() {
            let summ_price = price;
            if (summ_price > 0.0) {
                let filter = doc! { "id": p.id.clone() };
                let update = doc! { "$set": doc! {"status": 60,"part_price_sum": summ_price} };
                let res = self
                    .active_zip_requests_collection
                    .update_one(filter, update)
                    .await;
            }
        }
    }
    pub async fn q_create_contract(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            let update = doc! { "$set": doc! {"status": 70} };
            let res = self
                .active_zip_requests_collection
                .update_one(filter, update)
                .await;
        }
    }
    pub async fn q_create_supp_contract(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            let update = doc! { "$set": doc! {"status": 80} };
            let res = self
                .active_zip_requests_collection
                .update_one(filter, update)
                .await;
        }
    }
    pub async fn q_create_supp_payments(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            let update = doc! { "$set": doc! {"status": 90} };
            let res = self
                .active_zip_requests_collection
                .update_one(filter, update)
                .await;
        }
    }
    pub async fn q_create_supp_delivery(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            let update = doc! { "$set": doc! {"status": 100} };
            let res = self
                .active_zip_requests_collection
                .update_one(filter, update)
                .await;
        }
    }
    pub async fn q_create_post_processes(&self, parts: Vec<ReqItem>) {
        for p in parts.iter() {
            let filter = doc! { "id": p.id.clone() };
            let update = doc! { "$set": doc! {"status": 110} };
            let res = self
                .active_zip_requests_collection
                .update_one(filter, update)
                .await;
        }
    }
}
