use seed::{prelude::*, *};

use typebinder::{
    contexts::type_solving::TypeSolvingContextBuilder,
    error::TsExportError,
    exporters::Exporter,
    macros::context::MacroSolvingContext,
    path_mapper::PathMapper,
    pipeline::{module_step::ModuleStep, Pipeline},
    step_spawner::PipelineStepSpawner,
    syn,
};

fn typebinder_pass(input: &str) -> Result<String, TsExportError> {
    let pipeline_step_spawner = StringInputReader::new(input);
    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();
    let macro_context = MacroSolvingContext::default();
    let path_mapper = PathMapper::default();
    let mut output = String::default();
    Pipeline {
        pipeline_step_spawner,
        exporter: StringOutputter::new(&mut output),
        path_mapper,
    }
    .launch(&solving_context, &macro_context)?;
    Ok(output)
}

pub struct StringInputReader<'a> {
    input: &'a str,
}

impl<'a> StringInputReader<'a> {
    pub fn new(input: &'a str) -> Self {
        StringInputReader { input }
    }
}

pub struct StringOutputter<'a> {
    output: &'a mut String,
}

impl<'a> StringOutputter<'a> {
    pub fn new(output: &'a mut String) -> Self {
        StringOutputter { output }
    }
}
impl<'a> PipelineStepSpawner for StringInputReader<'a> {
    type Error = TsExportError;

    fn create_process(
        &self,
        path: typebinder::syn::Path,
    ) -> Result<Option<typebinder::pipeline::module_step::ModuleStep>, Self::Error> {
        let ast = syn::parse_file(self.input)?;
        Ok(Some(ModuleStep::new(path, ast.items, "")))
    }
}

impl<'a> Exporter for StringOutputter<'a> {
    type Error = TsExportError;

    fn export_module(
        &mut self,
        process_result: typebinder::pipeline::module_step::ModuleStepResultData,
    ) -> Result<(), Self::Error> {
        let out: String = process_result
            .imports
            .into_iter()
            .map(|statement| format!("{}\n", statement))
            .chain(
                process_result
                    .exports
                    .into_iter()
                    .map(|stm| stm.to_string()),
            )
            .collect();
        *self.output = out;

        Ok(())
    }
}



// Send Msg::OnTick every 200ms
const TICK_PERIOD: u32 = 200;

// Wait 600ms of no changes before trying to compile
const WAIT_TIME: u32 = 600;

// Number of ticks since the last change, be an attempt to compile
const WAIT_TICKS: u32 = WAIT_TIME / TICK_PERIOD;

#[derive(Debug)]
pub struct Model {
    source: String,
    output: Output,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum State {
    // No changes after the last compilation
    Clean,
    // Number of ticks elapsed since the last compilation
    Dirty { ticks: u32 },
}

#[derive(Debug)]
struct Output {
    typescript_code: String,
    error: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
    ChangeInput(String),
    Run,
    OnTick,
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::interval(TICK_PERIOD, || Msg::OnTick));
    Model {
        source: "".to_string(),
        output: Output {
            typescript_code: "".to_string(),
            error: None,
        },
        state: State::Clean,
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeInput(new_source) => {
            model.source = new_source;
            model.state = State::Dirty { ticks: 0 };
        },
        Msg::Run => {
            let output = typebinder_pass(&model.source);
            match output {
                Ok(ts_code) => {
                    model.output = Output {
                        typescript_code: format_typescript(ts_code),
                        error: None,
                    };
                },
                Err(e) => model.output.error = Some(e.to_string()),
            }
            model.state = State::Clean;
        }
        Msg::OnTick => {
            match model.state {
                State::Clean => (),
                State::Dirty { ticks } if ticks < WAIT_TICKS => {
                    model.state = State::Dirty { ticks: ticks + 1 };
                },
                State::Dirty { .. } => {
                    orders.send_msg(Msg::Run);
                }
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["columns"],
        div![
            C!["column"],
            div!["Rust input"],
            textarea![
                C!["textarea"],
                attrs! {
                    At::Value => model.source,
                    At::Rows => "20",
                },
                input_ev(Ev::Input, Msg::ChangeInput),
            ],
        ],
        div![
            C!["column"],
            div!["TypeScript Output"],
            pre![code![&model.output.typescript_code]],
            view_error(&model.output.error),
        ],
    ]
}

fn view_error(error: &Option<String>) -> Option<Node<Msg>> {
    error.as_ref().map(|message| {
        div![
            C!["notification is-danger"],
            message
        ]
    })
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    //App::start("app", init, update, view);
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logging");
    App::start("app", init, update, view);
}

fn format_typescript(code: String) -> String {
    use std::path::Path;
    use dprint_plugin_typescript::{
        configuration::{ConfigurationBuilder},
        format_text,
    };

    let config = ConfigurationBuilder::new()
        .line_width(120)
        .prefer_hanging(true)
        .prefer_single_line(false)
        .build();

    // dummy path to satisfy format_text() interface
    let path = Path::new("output.ts");
    let result = format_text(&path, &code, &config);

    match result {
        Ok(formatted_code) => formatted_code,
        Err(_) => code,
    }
}
