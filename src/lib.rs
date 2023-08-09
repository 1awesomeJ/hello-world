use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let body_string = String::from_utf8_lossy(&body);
    let body_json: Value = match serde_json::from_str(&body_string) {
	Ok(json) => json,
	Err(_) => {
	    send_response(
		400,
		vec![(String::from("content-type"), String::from("text/plain"))], "Invalid JSON body".as_bytes().to_vec(),
	);
	return;
	}
    };

    let username = body_json.get("username").and_then(Value::as_str).unwrap_or("");
    let password = body_json.get("password").and_then(Value::as_str).unwrap_or("");

    if username.is_empty() || password.is_empty() {
        send_response(
            400,
	    vec![(String::from("content-type"), String::from("text/plain"))],
	    "Welcome to our flow-function API! Please provide both a username and a password in the request body".as_bytes().to_vec(),
         );
	return;
    }
    
    let mut score = 0;

    let mut conditions = vec![
 	"Is at least 8 characters long",
        "Starts with an uppercase letter",
        "Contains both an alphabet character and a number",
        "Contains a special character",
    ];
   
    if password.len() >= 8 {
	score += 25;
	conditions.retain(|&x| x != "Is at least 8 characters long");
    }

    if password.chars().next().unwrap().is_uppercase() {
	score += 25;
	conditions.retain(|&x| x != "Starts with an uppercase letter");
    }

    if password.chars().any(|c| c.is_alphabetic()) && password.chars().any(|c| c.is_numeric()) {
	score += 25;
	conditions.retain(|&x| x != "Contains both an alphabet character and a number");
     }

    if password.chars().any(|c| !c.is_alphanumeric()) {
	score += 25;
	conditions.retain(|&x| x != "Contains a special character");
    }

    let login_status = if score == 100 { "is successful!" } else { "has failed!" };   

    let conditions_string = conditions.join("\n");

    let resp = format!("Hello {}, we need your password to meet 4 conditions. \nYour current password is {}% satisfactory.\n The oustanding condition(s) is/are: \n >>> {},\n therefore,*** your login {}. ***\n\n{}\n\n",
    	username,
	score,
	conditions_string,
	login_status,
	if score < 100 { "***Please try again with a password that matches the above listed condition(s)" } else { "" }
	);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/plain"))],
        resp.as_bytes().to_vec(),
    );
}
