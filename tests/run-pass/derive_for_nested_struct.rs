#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;
extern crate rocket_extra_codegen;

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::Request;

use rocket_extra_codegen::FromRequest;

#[derive(FromRequest)]
pub struct TestService {
    _field: TestField,
}

#[derive(FromRequest)]
pub struct DeepBooksService {
    _field: TestService,
}

#[derive(FromRequest)]
struct TestField;

fn main() {}
