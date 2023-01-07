#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct DataSecurity {
#[serde(rename = "data_identifier")]
data_identifier: String,
#[serde(rename = "data_owner")]
data_owner: String,
#[serde(rename = "data_custodian")]
data_custodian: String,
}

fn main() {
let data_security = DataSecurity {
data_identifier: String::from("12345"),
data_owner: String::from("John Smith"),
data_custodian: String::from("Jane Smith"),
};

let serialized_data = serde_json::to_string(&data_security).unwrap();
println!("{}", serialized_data);

let deserialized_data: DataSecurity = serde_json::from_str(&serialized_data).unwrap();
println!("{:?}", deserialized_data);
}
