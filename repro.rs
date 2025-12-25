trait Component {
    fn render(&self);
}

struct Wrapper<T>(T);

impl<T: Component> Wrapper<T> {
    fn render(&self) {
        self.0.render();
    }
}

impl<T: Component + ?Sized> Component for &T {
    fn render(&self) {
        (**self).render();
    }
}

struct MyComponent;
impl Component for MyComponent {
    fn render(&self) {}
}

fn from_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn container(children: String) {
    // This mimics the macro expansion
    from_fn(move || {
        // This mimics {children} expansion
        Wrapper(&children).render();
    });
}

fn main() {
    container(String::from("hello"));
}
