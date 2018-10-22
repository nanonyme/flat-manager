#![allow(proc_macro_derive_resolution_fallback)]

use chrono;

#[derive(Serialize, Queryable, Debug)]
pub struct Build {
    pub id: i32,
    pub is_published: bool,
    pub created: chrono::NaiveDateTime,
    pub repo_state: i16
}


#[derive(Serialize, Queryable, Debug)]
pub struct BuildLog {
    pub id: i32,
    pub build_id: i32,
    pub log_type: i16,
}

#[derive(Serialize, Queryable, Debug)]
pub struct BuildRef {
    pub id: i32,
    pub build_id: i32,
    pub ref_type: i16,
    pub ref_name: String,
    pub commit: String,
}