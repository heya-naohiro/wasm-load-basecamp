wit_bindgen::generate!("host-world");

struct Component;

impl Guest for Component {
    fn run() {
        print("called run");
    }
}

export!(Component);
