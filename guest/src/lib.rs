wit_bindgen::generate!("host-world" in "../wit/");

struct Component;

impl Guest for Component {
    /// Say hello!
    fn run() {
        print("Hello");
    }
}

export!(Component);
