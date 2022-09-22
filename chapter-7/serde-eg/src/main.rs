use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    lat: f64,
    lon: f64,
}

fn main() {
    let ams = City {
        name: String::from("Amsterdam"),
        population: 907_000,
        lat: 4.9,
        lon: 52.3667,
    };

    let as_json = to_json(&ams).unwrap();
    let as_cbor = to_cbor(&ams).unwrap();
    let as_binc = to_bincode(&ams).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_binc);

    println!("json (utf8):\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor (utf8):\n{}\n", String::from_utf8_lossy(&as_cbor));
    println!("bincode (utf8):\n{}\n", String::from_utf8_lossy(&as_binc));
}
