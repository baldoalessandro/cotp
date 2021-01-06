use crate::database_loader;
use serde_json;
use serde::{Deserialize, Serialize};
use crate::otp::make_totp;

#[derive(Serialize, Deserialize)]
struct JsonResult{
    index: usize,
    issuer: String,
    label: String,
    otp_code: String,
}

impl JsonResult {
    pub fn new(index: usize, issuer: String, label: String,otp_code: String) -> JsonResult {
        JsonResult{
            index: index, 
            issuer: issuer,
            label: label,
            otp_code: otp_code
        }
    }
}

pub fn read_codes() -> Result<Vec<database_loader::OTPElement>,String>{
    match database_loader::read_from_file(){
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

pub fn show_codes(elements: &Vec<database_loader::OTPElement>) -> usize{
    //let elements;
    for i in 0..elements.len() {
        print_totp(i,&elements[i]);
    }
    elements.len()
}

fn print_totp(i: usize,element: &database_loader::OTPElement){
    if element.issuer() != ""{
        println!("{}) {} - {}: {}",i+1,element.issuer(),element.label(),get_good_otp_code(&element));
    }else{
        println!("{}) {}: {}",i+1,element.label(),get_good_otp_code(&element));
    }
}

fn get_good_otp_code(element: &database_loader::OTPElement) -> String {
    let otp = make_totp(
        &element.secret(), //we have replaced '=' in this method
               element.period(), 0).unwrap();
    let mut s_otp = otp.to_string();

    while s_otp.len() < element.digits() as usize {
        s_otp = String::from("0") + &s_otp;
    }
    s_otp
}

pub fn get_json_results() -> Result<String,String>{
    let elements: Vec<database_loader::OTPElement>;

    match database_loader::read_from_file(){
        Ok(result) => elements = result,
        Err(e) => return Err(e)
    }
    let mut results: Vec<JsonResult> = Vec::new();

    if elements.len() == 0{
        return Err(String::from("there are no elements in your database, type \"cotp -h\" to get help"));
    }

    for i in 0..elements.len() {
        let otp_code = get_good_otp_code(&elements[i]);
        results.push(JsonResult::new(i+1,elements[i].issuer(),elements[i].label(),otp_code))
    }

    let json_string: &str = &serde_json::to_string_pretty(&results).unwrap();

    Ok(json_string.to_string())
}