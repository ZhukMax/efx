use eframe::{NativeOptions, egui};
use efx::efx;

fn main() -> eframe::Result<()> {
    let native = NativeOptions::default();
    eframe::run_native(
        "EFx Sandbox",
        native,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App {
    counter: i32,
    input: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            let _ = efx!(
                ui,
                r#"
                <Column gap="8">
                    <Label size="20" bold="true">EFx sandbox</Label>
                    <Separator/>
                </Column>
            "#
            );

            // Increment/decrement buttons - catch Response
            ui.horizontal(|ui| {
                let inc = efx!(ui, r#"<Button tooltip="Increment">+1</Button>"#);
                if inc.clicked() {
                    self.counter += 1;
                }

                let dec = efx!(ui, r#"<Button tooltip="Decrement">-1</Button>"#);
                if dec.clicked() {
                    self.counter -= 1;
                }
            });

            // Dynamic text via {expr}
            let _ = efx!(ui, r#"<Label>Counter: {self.counter}</Label>"#);

            // Input field (binding directly to the state field)
            let _ = efx!(ui, r#"<TextField value="self.input" hint="type here…"/>"#);

            // Scrolling + different tags
            let _ = efx!(
                ui,
                r##"
                <ScrollArea axis="vertical" max-height="160" always-show="true" id="demo-log">
                  <Heading level="1">Main title</Heading>
                  <Heading level="2" color="#66CCFF">Section</Heading>
                  <Heading level="3" size="14" tooltip="Subheading">Small note</Heading>
                  <Column gap="6">
                    <Label monospace="true">You typed: {self.input.clone()}</Label>
                    <Row gap="8">
                      <Hyperlink url="https://efxui.com" tooltip="Project site"/>
                      <Hyperlink url="help:about" open_external="false">About</Hyperlink>
                    </Row>
                    <Separator/>
                    <Row gap="10" wrap="true">
                      <Button fill="#333333" rounding="8">A</Button>
                      <Button frame="false">B</Button>
                      <Button min_width="100" tooltip="Wide">Wide</Button>
                    </Row>
                  </Column>
                </ScrollArea>
            "##
            );

            // SidePanel with other tags
            let _ = efx!(ui, r##"
                <SidePanel side="left" id="nav" default-width="240" min-width="160" resizable="true" fill="#15151A">
                  <Column gap="8" padding="8">
                    <Label size="16" bold="true">Navigation</Label>
                    <Separator/>
                    <Button frame="false">Home</Button>
                    <Button frame="false">Projects</Button>
                    <Button frame="false">Settings</Button>
                  </Column>
                </SidePanel>
            "##);

            let _ = efx!(ui, r##"
                <TopPanel id="appbar" default-height="36" fill="#15151A" stroke-width="1" stroke-color="#262A33">
                  <Row gap="8" padding="6">
                    <Label bold="true">EFx App</Label>
                    <Separator/>
                    <Button frame="false">File</Button>
                    <Button frame="false">Edit</Button>
                    <Button frame="false">View</Button>
                  </Row>
                </TopPanel>
            "##);

            let _ = efx!(ui, r##"
                <BottomPanel id="console" default-height="200" resizable="true" fill="#0F1116">
                  <ScrollArea axis="vertical" max-height="180" id="console-scroll">
                    <Column gap="4" padding="6">
                      <Label monospace="true">[12:00:01] Ready.</Label>
                      <Label monospace="true">[12:00:02] Build succeeded.</Label>
                    </Column>
                  </ScrollArea>
                </BottomPanel>
            "##);
        });
    }
}
