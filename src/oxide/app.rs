
pub trait Application {
    fn run(&self);
}


pub struct OxideApp {

}

impl OxideApp {
    pub fn new() -> OxideApp {
        OxideApp{}
    }
}


impl Application for OxideApp {
    fn run(&self) {
        loop {};
    }
}