use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

pub mod syncdb;
pub mod asyncdb;

pub const UNKNOWN_ITEM: &str = "UNKNOWN";

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqItem {
    pub id: String,
    pub client_name: String,
    pub request_id: String,
    pub request_day: i32,
    pub request_month: i32,
    pub request_year: i32,
    pub target: String,
    pub target_place: String,
    pub imo: String,
    pub eq_type: String,
    pub product_type: String,
    pub factory_name: String,
    pub serials: Vec<String>,
    pub serial: String,
    pub customer_person: String,
    pub supplier_person: String,
    pub incoterms_rule: String,
    pub valid_day: i32,
    pub valid_month: i32,
    pub valid_year: i32,
    pub part_no: String,
    pub part_descr: String,
    pub part_id: String,
    pub part_unit: String,
    pub part_qty: f32,
    pub part_price: f32,
    pub part_price_sum: f32,
    pub part_delivery: i32,
    pub part_currency: String,
}
impl ReqItem {}

impl Display for ReqItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReqItem {{ id: {}, client_name: {}, request_id: {}, request_day: {}, request_month: {}, request_year: {}, target: {}, target_place: {}, imo: {}, eq_type: {}, product_type: {}, factory_name: {}, serials: {:?}, serial: {}, customer_person: {}, supplier_person: {}, incoterms_rule: {}, valid_day: {}, valid_month: {}, valid_year: {}, part_no: {}, part_descr: {}, part_id: {}, part_unit: {}, part_qty: {}, part_price: {}, part_price_sum: {}, part_delivery: {}, part_currency: {} }}", self.id, self.client_name, self.request_id, self.request_day, self.request_month, self.request_year, self.target, self.target_place, self.imo, self.eq_type, self.product_type, self.factory_name, self.serials, self.serial, self.customer_person, self.supplier_person, self.incoterms_rule, self.valid_day, self.valid_month, self.valid_year, self.part_no, self.part_descr, self.part_id, self.part_unit, self.part_qty, self.part_price, self.part_price_sum, self.part_delivery, self.part_currency)
    }
}
impl Debug for ReqItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReqItem {{ id: {}, client_name: {}, request_id: {}, request_day: {}, request_month: {}, request_year: {}, target: {}, target_place: {}, imo: {}, eq_type: {}, product_type: {}, factory_name: {}, serials: {:?}, serial: {}, customer_person: {}, supplier_person: {}, incoterms_rule: {}, valid_day: {}, valid_month: {}, valid_year: {}, part_no: {}, part_descr: {}, part_id: {}, part_unit: {}, part_qty: {}, part_price: {}, part_price_sum: {}, part_delivery: {}, part_currency: {} }}", self.id, self.client_name, self.request_id, self.request_day, self.request_month, self.request_year, self.target, self.target_place, self.imo, self.eq_type, self.product_type, self.factory_name, self.serials, self.serial, self.customer_person, self.supplier_person, self.incoterms_rule, self.valid_day, self.valid_month, self.valid_year, self.part_no, self.part_descr, self.part_id, self.part_unit, self.part_qty, self.part_price, self.part_price_sum, self.part_delivery, self.part_currency)
    }
}
