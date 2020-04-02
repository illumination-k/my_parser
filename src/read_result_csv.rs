extern crate csv;
use std::error::Error;

use std::collections::HashMap;


pub fn parse_result_csv_core(path: &str, key: usize) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut ret: HashMap<String, HashMap<String, String>> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let uname = &record[0];
        let cell_bc = &record[1];
        if !ret.contains_key(uname) {
            let mut map: HashMap<String, String> = HashMap::new();
            let key_str = &record[key];
            map.insert(cell_bc.to_string(), key_str.to_string());
            ret.insert(uname.to_string(), map.clone());
        } else {
            if !ret.get(uname).unwrap().contains_key(cell_bc) {
                let mut map: HashMap<String, String> = HashMap::new();
                let key_str = &record[key];
                map.insert(cell_bc.to_string(), key_str.to_string());
                ret.insert(uname.to_string(), map.clone());
            } else {
                panic!("format may be invalid!");
            }
        }
    }
    Ok(ret)
}
