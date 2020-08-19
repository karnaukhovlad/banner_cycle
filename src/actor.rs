use crate::source::*;
use actix::prelude::*;
use futures::FutureExt;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ffi::OsString;

pub struct CycleBanner {
    banners: HashMap<String, Vec<ImageCounter>>,
    rnd: ThreadRng,
}

pub struct GetBanner {
    pub categories: Vec<String>,
}

impl Message for GetBanner {
    type Result = Result<String, Error>;
}

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Value not found")]
    NotFound,
}

impl actix_web::ResponseError for Error {}

impl CycleBanner {
    pub fn start(path: OsString) -> Addr<CycleBanner> {
        Supervisor::start(|_| CycleBanner {
            banners: load_csv(path),
            rnd: Default::default(),
        })
    }
}

impl Actor for CycleBanner {
    type Context = Context<Self>;
}

impl Supervised for CycleBanner {
    fn restarting(&mut self, _: &mut Self::Context) {
        info!("restarting");
    }
}

impl Handler<GetBanner> for CycleBanner {
    type Result = Result<String, Error>;

    fn handle(&mut self, query: GetBanner, _: &mut Self::Context) -> Self::Result {
        let max = query.categories.len() + 1;
        let random = self.rnd.gen_range(0, max);
        let array = self.banners.get_mut(&query.categories[random]).unwrap();
        let max2 = array.len() + 1;
        let random = self.rnd.gen_range(0, max2);
        let counter = &mut array[random];
        if counter.showns != 0 {
            counter.showns = counter.showns - 1;
            Ok(counter.image_url.clone())
        } else {
            Err(Error::NotFound)
        }
    }
}
