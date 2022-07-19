#![windows_subsystem = "windows"]

// use druid::im::Vector;
use druid::widget::{
    Axis, CrossAxisAlignment, Flex, Label, MainAxisAlignment, RadioGroup,// Split, TabInfo, TabsPolicy, Button
    Tabs, TabsEdge, TabsTransition, TextBox, ViewSwitcher,
};
use druid::{theme, AppLauncher, Color, FontFamily, FontStyle, FontWeight, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use instant::Duration;

// use druid::piet::{PietTextLayoutBuilder, TextStorage as PietTextStorage};
use druid::text::{Attribute, RichText,};

const TEXT: &str = r#"Contrary to what we would like to believe,
there is no such thing as a structureless group. Any group of people 
of whatever nature that comes together for any length of time for any purpose will inevitably structure itself in some fashion. The structure may be flexible; it may vary over time; it may evenly or unevenly distribute tasks, power and resources over the members of the group. But it will be formed regardless of the abilities, personalities, or intentions of the people involved. The very fact that we are individuals, with different talents, predispositions, and backgrounds makes this inevitable. Only if we refused to relate or interact on any basis whatsoever could we approximate structurelessness -- and that is not the nature of a human group.
This means that to strive for a structureless group is as useful, and as deceptive, as to aim at an "objective" news story, "value-free" social science, or a "free" economy. A "laissez faire" group is about as realistic as a "laissez faire" society; the idea becomes a smokescreen for the strong or the lucky to establish unquestioned hegemony over others. This hegemony can be so easily established because the idea of "structurelessness" does not prevent the formation of informal structures, only formal ones. Similarly "laissez faire" philosophy did not prevent the economically powerful from establishing control over wages, prices, and distribution of goods; it only prevented the government from doing so. Thus structurelessness becomes a way of masking power, and within the women's movement is usually most strongly advocated by those who are the most powerful (whether they are conscious of their power or not). As long as the structure of the group is informal, the rules of how decisions are made are known only to a few and awareness of power is limited to those who know the rules. Those who do not know the rules and are not chosen for initiation must remain in confusion, or suffer from paranoid delusions that something is happening of which they are not quite aware."#;


#[derive(Data, Clone, Lens)]
struct DynamicTabData {
    highest_tab: usize,
    removed_tabs: usize,
   // tab_labels: Vector<usize>,
}

impl DynamicTabData {
    fn new(highest_tab: usize) -> Self {
        DynamicTabData {
            highest_tab,
            removed_tabs: 0,
            // tab_labels: (1..=highest_tab).collect(),
        }
    }

    // fn add_tab(&mut self) {
    //     self.highest_tab += 1;
    //     // self.tab_labels.push_back(self.highest_tab);
    // }

    // fn remove_tab(&mut self, idx: usize) {
    //     if idx >= self.tab_labels.len() {
    //         tracing::warn!("Attempt to remove non existent tab at index {}", idx)
    //     } else {
    //         self.removed_tabs += 1;
    //         self.tab_labels.remove(idx);
    //     }
    // }

    // // This provides a key that will monotonically increase as interactions occur.
    // fn tabs_key(&self) -> (usize, usize) {
    //     (self.highest_tab, self.removed_tabs)
    // }
}

#[derive(Data, Clone, Lens)]
struct TabConfig {
    axis: Axis,
    edge: TabsEdge,
    transition: TabsTransition,
}

#[derive(Data, Clone, Lens)]
struct AppState {
    tab_config: TabConfig,
    advanced: DynamicTabData,
    first_tab_name: String,
    text: RichText,
}


pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Tabs")
        .window_size((700.0, 400.0));
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
            transition: Default::default(),
            
        },
        first_tab_name: "First tab".into(),
        advanced: DynamicTabData::new(2),
        text: text2,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    fn group<T: Data, W: Widget<T> + 'static>(text: &str, w: W) -> impl Widget<T> {
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(
                Label::new(text)
                    .background(theme::PLACEHOLDER_COLOR)
                    .expand_width(),
            )
            .with_default_spacer()
            .with_child(w)
            .with_default_spacer()
            .border(Color::WHITE, 0.5)
    }

    let axis_picker = group(
        "Tab bar axis",
        RadioGroup::column(vec![
            ("Horizontal", Axis::Horizontal),
            ("Vertical", Axis::Vertical),
        ])
        .lens(TabConfig::axis),
    );

    let cross_picker = group(
        "Tab bar edge",
        RadioGroup::column(vec![
            ("Leading", TabsEdge::Leading),
            ("Trailing", TabsEdge::Trailing),
        ])
        .lens(TabConfig::edge),
    );

    let transit_picker = group(
        "Transition",
        RadioGroup::column(vec![
            ("Instant", TabsTransition::Instant),
            (
                "Slide",
                TabsTransition::Slide(Duration::from_millis(250).as_nanos() as u64),
            ),
        ])
        .lens(TabConfig::transition),
    );

    let sidebar = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(axis_picker)
        .with_default_spacer()
        .with_child(cross_picker)
        .with_default_spacer()
        .with_child(transit_picker)
        .with_flex_spacer(1.)
        .fix_width(200.0)
        .lens(AppState::tab_config);

    let vs = ViewSwitcher::new(
        |app_s: &AppState, _| app_s.tab_config.clone(),
        |tc: &TabConfig, _, _| Box::new(build_tab_widget(tc)),
    );
    Flex::row().with_child(sidebar).with_flex_child(vs, 1.0)
}


fn build_tab_widget(tab_config: &TabConfig) -> impl Widget<AppState> {

    // let control_dynamic = Flex::column()
    //     .cross_axis_alignment(CrossAxisAlignment::Start)
    //     .with_child(Label::new("Control dynamic tabs"))
    //     .with_child(Button::new("Add a tab").on_click(|_c, d: &mut DynamicTabData, _e| d.add_tab()))
    //     .with_child(Label::new(|adv: &DynamicTabData, _e: &Env| {
    //         format!("Highest tab number is {}", adv.highest_tab)
    //     }))
    //     .with_spacer(20.)
    //     .lens(AppState::advanced);

    let first_static_tab = Flex::row()
        .with_child(Label::new("Rename tab:"))
        .with_child(TextBox::new().lens(AppState::first_tab_name));

    let main_tabs = Tabs::new()
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .with_tab(
            |app_state: &AppState, _: &Env| app_state.first_tab_name.to_string(),
            first_static_tab,
        )
        // .with_tab("Dynamic", control_dynamic)
        .with_tab("Page 3", Label::new(TEXT))
        .with_tab("Page 4", Label::new(""))
        .with_tab("Page 5", Label::new("Page 5 content"))
        .with_tab("Page 6", Label::new("Page 6 content"))
        .with_tab_index(1);

    // Split::rows(main_tabs, dyn_tabs).draggable(true)
    main_tabs
}
