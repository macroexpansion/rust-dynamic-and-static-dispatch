trait Log {
    fn log(&self);
}

struct Data {
    data: String,
}

impl Log for Data {
    #[inline(always)]
    fn log(&self) {
        println!("{}", self.data);
    }
}

fn run_static<T: Log>(logger: T) {
    logger.log();
}

fn run_dynamic(logger: Box<dyn Log>) {
    logger.log();
}

fn run_dynamic_ref(logger: &dyn Log) {
    logger.log();
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let static_logger = Data {
            data: String::from("static"),
        };
        run_static(static_logger);

        let dyn_logger = Box::new(Data {
            data: String::from("dynamic"),
        });
        run_dynamic_ref(dyn_logger.as_ref());
        run_dynamic(dyn_logger);
    }
}
