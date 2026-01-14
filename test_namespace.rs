mod library_page {
    pub fn render() { println!("render"); }
}

fn library_page() {
    println!("wrapper");
    library_page::render();
}

fn main() {
    library_page();
}
