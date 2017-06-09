
use fmt::Debug;

pub trait Error: Debug {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error> {
        None
    }
}
