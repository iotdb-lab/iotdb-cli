use crate::common;
use crate::opt::SubCmd;
use crate::VERSION;

impl SubCmd {
    /// Print help info
    pub fn usage(&self) {
        common::print_usage();
    }

    #[allow(dead_code)]
    pub fn update(&self) {
        println!("Update to {:?}", VERSION)
    }

    #[allow(dead_code)]
    pub fn download_latest(&self) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn replace_binary(&self) {
        todo!()
    }
}
