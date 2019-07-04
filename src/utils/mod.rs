use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyList};
use rand::Rng;

pub fn send_sms(phone_num: String, code: String) -> Result<(), String> {
    dotenv::dotenv().ok();
    let code = vec![code];
    let app_id = std::env::var("SMS_APPID").map_err(|_| "SMS_APPID key must be set")?;
    let app_key = std::env::var("SMS_APPKEY").map_err(|_| "SMS_APPKEY key must be set")?;
    let template_id = std::env::var("SMS_TEMPLATE_ID").map_err(|_| "SMS_TEMPLATE_ID key must be set")?;
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sms_single_sender = py.import("qcloudsms_py").unwrap();
    let args = PyTuple::new(py, &[&app_id, &app_key]);
    let ssender = sms_single_sender.call1("SmsSingleSender", args).unwrap();
    let ssender = ssender.to_object(py);
    let args1_list = PyList::empty(py);
    args1_list.append(86).unwrap();
    args1_list.append(&phone_num).unwrap();
    args1_list.append(template_id.parse::<i32>().unwrap()).unwrap();
    args1_list.append(code).unwrap();
    let args1 = PyTuple::new(py, &[args1_list.get_item(0), args1_list.get_item(1), args1_list.get_item(2), args1_list.get_item(3)]);
    let result = ssender.call_method1(py, "send_with_param", args1).unwrap();
    let result: String = result.call_method0(py, "__str__").unwrap().extract(py).unwrap();
    if result.contains("OK") {
        Ok(())
    }
    else {
        Err("短信发送失败".to_string())
    }
}

pub fn gen_sms_code() -> String {
    let mut rng = rand::thread_rng();
    let mut result: String = String::new();
    for _ in 0..4 {
        let i: u8 = rng.gen_range(0, 10);
        result += &i.to_string();
    }
    result
}