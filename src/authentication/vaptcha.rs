use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyList, PyDict};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateToken {
    pub phone: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePayload {
    id: String,
    secretkey: String,
    scene: Option<String>,
    token: String,
    ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidResponse {
    success: i32,
    score: i32,
    msg: String,
}

impl ValidatePayload {
    pub fn new(token: String) -> Self {
        dotenv::dotenv().ok();
        let vid = std::env::var("VAPTCHA_VID")
            .expect("VAPTCHA_VID must be set");
        let key = std::env::var("VAPTCHA_KEY")
            .expect("VAPTCHA_KEY must be set");
        Self {
            id: vid,
            secretkey: key,
            scene: None,
            token,
            ip: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        let url = "http://api.vaptcha.com/v2/validate";
        let gil = Python::acquire_gil();
        let py = gil.python();
        let requests = py.import("requests").expect("import requests package failed");
        let headers = PyDict::new(py);
        let data = PyDict::new(py);
        data.set_item("id", &self.id).unwrap();
        data.set_item("secretkey", &self.secretkey).unwrap();
        data.set_item("token", &self.token).unwrap();
        let args_list = PyList::empty(py);
        args_list.append(url).unwrap();
        args_list.append(headers).unwrap();
        args_list.append(data).unwrap();
        let args = PyTuple::new(py, &[args_list.get_item(0)]);
        let kwargs = PyDict::new(py);
        kwargs.set_item("headers", args_list.get_item(1)).unwrap();
        kwargs.set_item("data", args_list.get_item(2)).unwrap();
        let response = requests.call("post", args, Some(kwargs)).unwrap();
        let response = response.to_object(py);
        let response: String = response.getattr(py, "text").unwrap().extract(py).unwrap();
        let response = serde_json::from_str::<ValidResponse>(&response).unwrap();
        if response.success == 1 {
            true
        }
        else {
            false
        }
    }
}