trait Log {
    fn log(&self);
}

#[derive(Clone)]
struct Data {
    data: String,
}

impl Log for Data {
    #[inline(always)]
    fn log(&self) {
        println!("{}", self.data);
    }
}

// static dispatch: generate functions for each type at compile time
// fast but binary size increase
fn run_static<T: Log>(logger: T) {
    logger.log();
}

fn run_static_impl(logger: impl Log) {
    logger.log();
}

// dynamic dispatch: generate functions at runtime
// a little bit slower because of vtable but smaller binary size
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
        run_static(static_logger.clone());
        run_static_impl(static_logger);

        let dyn_logger = Box::new(Data {
            data: String::from("dynamic"),
        });
        run_dynamic_ref(dyn_logger.as_ref());
        run_dynamic(dyn_logger);
    }
}
