use csv::ReaderBuilder;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub struct ImageCounter {
    pub showns: i32,
    pub image_url: String,
}

impl Ord for ImageCounter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.showns.cmp(&other.showns)
    }
}

pub fn load_csv(path: OsString) -> HashMap<String, Vec<ImageCounter>> {
    let mut map: HashMap<String, Vec<ImageCounter>> = HashMap::with_capacity(1000);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .has_headers(false)
        .from_path(&path)
        .unwrap();
    let mut iter = rdr.records();
    iter.for_each(|record_line| {
        let mut record = record_line.unwrap();
        record.trim();
        let mut rec_iter = record.iter();
        let image_url = rec_iter.next().unwrap();
        let showns = rec_iter.next().unwrap();
        rec_iter.for_each(|category| {
            let image_counter = ImageCounter {
                showns: showns.parse().unwrap(),
                image_url: image_url.to_string(),
            };
            if let Some(cell) = map.get_mut(category) {
                cell.push(image_counter);
            } else {
                map.insert(category.to_string(), vec![image_counter]);
            };
        });
    });
    map
}
