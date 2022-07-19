#![windows_subsystem = "windows"]

// use druid::im::Vector;
use druid::widget::{
    Axis, CrossAxisAlignment, Flex, Label, MainAxisAlignment, RadioGroup,// Split, TabInfo, TabsPolicy, Button
    Tabs, TabsEdge, TabsTransition, TextBox, ViewSwitcher, Painter
};
use druid::{theme, AppLauncher, Color, FontFamily, FontStyle, FontWeight, Data, Env, Lens, Widget, WidgetExt, WindowDesc, LinearGradient, UnitPoint};
use instant::Duration;
use druid::kurbo::Circle;
// use druid::piet::{PietTextLayoutBuilder, TextStorage as PietTextStorage};
use druid::text::{Attribute, RichText,};
use druid::widget::prelude::*;
const DARK_GREY: Color = Color::grey8(0x3a);
const DARKER_GREY: Color = Color::grey8(0x11);
const LIGHTER_GREY: Color = Color::grey8(0xbb);

const TEXT: &str = r#"March 2021 – Today 
Software Engineer Rust / System Engineer- TMT Analysis
- Rust IT and Telecom Systems Developer,
- Multithreading programming and API type applications,
- Implementation of API-Vendors,
- Creation of CRM functions,
- Implementation of an SQL documentary platform for the sales department,
- Creation of an activity increment file 
(Objective  improving the follow-up),
- Creation of a summary and activity monitoring file for the 
finance department (Objective  improvement of invoicing monitoring)
"#;

const TEXT2: &str = r#"July 2020 – March 2021 
Software Engineer Python/Django - Freelance
- Application Python Django 
- Web déploiement Heroku/Linux
- Project type e-commerce
"#;

const TEXT3: &str = r#"May 2018 – July 2020 
Pilote Etude Architecture Automobile - Expleo pour PSA
- Creation of an offshore platform in Romania of 10 people for 
Stellantis (former Peugeot/Citroën),
- Writing E&E specifications for ECU,
- Realization of SW design (White-Box), definition of HW/SW interface 
requirements and different integration methods,
- Analysis of non-conformities / proposals for technical solutions,
- Security, industrialization, and after-sales consistency matrix,
- Analysis and implementation of corrective solutions in serial life.
"#;

const TEXT4: &str = r#"May 2018 – July 2020 
Stage System Engineer – Safran Electronics & Defense 
- Bibliographic study on the Battery Management System,
- Characterization of a battery by impedance spectroscopy,
- Creation of an electronic circuit for electric wave emission,
- Control of the power consumption of an infrared binocular,
- Development of an HMI to simulate the power consumption profiles 
of the binoculars.
"#;

#[derive(Data, Clone, Lens)]
struct TabConfig {
    axis: Axis,
    edge: TabsEdge,
    transition: TabsTransition,
}

#[derive(Data, Clone, Lens)]
struct AppState {
    tab_config: TabConfig,
    first_tab_name: String,
    text: RichText,
}


pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Tabs")
        .window_size((790.0, 400.0));
    let text2 = RichText::new(TEXT.into())
        .with_attribute(0..9, Attribute::text_color(Color::rgb(1.0, 0.2, 0.1)))
        .with_attribute(0..9, Attribute::size(24.0))
        .with_attribute(0..9, Attribute::font_family(FontFamily::SERIF))
        .with_attribute(194..239, Attribute::weight(FontWeight::BOLD))
        .with_attribute(764.., Attribute::size(12.0))
        .with_attribute(764.., Attribute::style(FontStyle::Italic));
    // create the initial app state
    let initial_state = AppState {
        tab_config: TabConfig {
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
            transition: TabsTransition::Slide(Duration::from_millis(250).as_nanos() as u64), 
        },
        first_tab_name: "First tab".into(),
        text: text2,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {

    let gradient = LinearGradient::new(
        UnitPoint::TOP_LEFT,
        UnitPoint::BOTTOM_RIGHT,
        (DARKER_GREY, LIGHTER_GREY),
    );

    let polka_dots = Painter::new(|ctx, _, _| {
        let bounds = ctx.size().to_rect();
        let dot_diam = bounds.width().max(bounds.height()) / 20.;
        let dot_spacing = dot_diam * 1.8;
        for y in 0..((bounds.height() / dot_diam).ceil() as usize) {
            for x in 0..((bounds.width() / dot_diam).ceil() as usize) {
                let x_offset = (y % 2) as f64 * (dot_spacing / 2.0);
                let x = x as f64 * dot_spacing + x_offset;
                let y = y as f64 * dot_spacing;
                let circ = Circle::new((x, y), dot_diam / 2.0);
                let purp = Color::rgb(0.3, 0.22, 0.76);
                ctx.fill(circ, &purp);
            }
        }
    });

    let sidebar = Flex::column()
    .with_flex_child(Flex::column()
        .with_child(Label::new("Merry de Lassus
Software Engineer Rust / 
System Engineer
        ")
        // .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .center()
        .background(polka_dots)
        .border(DARK_GREY, 4.0)
        .padding(10.0),
    ).with_child(Label::new("13 rue Guillemette Faussart
Suresnes 92150 – France
     
(+33)635909228 
merry2lassus@gmail.com
    ")
        .center()
        .background(gradient)
        .border(DARK_GREY, 4.0)
        .padding(10.0),
    ).with_child(Label::new("Contact test test"))


, 1.)
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_spacer(1.)
        .fix_width(220.0)
        .lens(AppState::tab_config);

    let vs = ViewSwitcher::new(
        |app_s: &AppState, _| app_s.tab_config.clone(),
        |tc: &TabConfig, _, _| Box::new(build_tab_widget(tc)),
    );
    Flex::row()
        .with_child(sidebar)
        .with_flex_child(vs, 1.0)
        // .with_flex_child(Flex::row(), 1.)
}


fn build_tab_widget(tab_config: &TabConfig) -> impl Widget<AppState> {

    let main_tabs = Tabs::new()
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .with_tab("TMT Analysis", Label::new(TEXT))
        .with_tab("Freelance", Label::new(TEXT2))
        .with_tab("Expleo pour PSA", Label::new(TEXT3))
        .with_tab("Safran Electronics & Defense", Label::new(TEXT4))
        .with_tab_index(1);
    main_tabs
}
