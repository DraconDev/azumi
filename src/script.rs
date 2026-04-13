use crate::Component;

pub struct AzumiScript;

impl Component for AzumiScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<script>{}</script>",
            crate::AZUMI_JS.replace("</script>", r"<\/script>")
        )
    }
}
