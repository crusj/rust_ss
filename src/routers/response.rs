pub struct Response {
    headers: Vec<(String, String)>,
    data: Option<String>,
}

impl Response {
    pub fn set_header(&mut self, key: String, value: String) -> &mut Response {
        self.headers.push((key, value));
        self
    }

    pub fn set_data(&mut self, data: String) -> &mut Response {
        self.data = Some(data);
        self
    }

    pub fn to_stream(&mut self) -> String {
        let data_size = match &self.data {
            Some(data) => data.len(),
            None => 0,
        };
        self.set_header("Content-Type".to_string(), "application/json".to_string());
        if data_size > 0 {
            self.set_header("Content-Length".to_string(), data_size.to_string());
        };

        // status
        let mut resp = String::from("HTTP/1.1 200 OK\r\n");

        // header
        for (k, v) in &self.headers {
            resp.push_str(&k);
            resp.push_str(": ");
            resp.push_str(&v);
            resp.push_str("\r\n");
        }
        // 空行
        resp.push_str("\r\n");

        if let Some(ref data) = &self.data {
            resp.push_str(data);
        };

        resp
    }
}

pub fn new() -> Response {
    Response {
        headers: Vec::new(),
        data: None,
    }
}
