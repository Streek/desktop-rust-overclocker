use iced::{
    button, text_input, Align, Application, Button, Clipboard, Column, Command, Element, Settings,
    TextInput,
};
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};

struct State {
    Nvml: Nvml,
    core_speed: i32,
    memory_speed: i32,
    video_speed: i32,
    core_input: text_input::State,
    memory_input: text_input::State,
    video_input: text_input::State,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let Nvml = Nvml::init().unwrap();
        let device = Nvml.device_by_index(0)?;
        let clock_speeds = device.clock_info(Cl
        let core = clock_speeds.core;
        let memory = clock_speeds.memory;
        let video = clock_speeds.video;
        (
            Self {
                Nvml,
                core_speed: core,
                memory_speed: memory,
                video_speed: video,
                core_input: text_input::State::new(),
                memory_input: text_input::State::new(),
                video_input: text_input::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("GPU Overclocking")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let core_input = TextInput::new(
            &mut self.core_input,
            "Core Speed",
            &self.core_speed.to_string(),
            |core_speed| {
                self.core_speed = core_speed.parse().unwrap();
            },
        )
        .padding(10);

        let memory_input = TextInput::new(
            &mut self.memory_input,
            "Memory Speed",
            &self.memory_speed.to_string(),
            |memory_speed| {
                self.memory_speed = memory_speed.parse().unwrap();
            },
        )
        .padding(10);

        let video_input = TextInput::new(
            &mut self.video_input,
            "Video Speed",
            &self.video_speed.to_string(),
            |video_speed| {
                self.video_speed = video_speed.parse().unwrap();
            },
        )
        .padding(10);

        let apply_button = Button::new(&mut button::State::new(), "Apply").on_press(|| {
            let device = self.Nvml.device_by_index(0).unwrap();
            device
                .set_clock(nvml_wrapper::ClockType::Graphics, self.core_speed)
                .unwrap();
            device
                .set_clock(nvml_wrapper::ClockType::Memory, self.memory_speed)
                .unwrap();
        });

        Column::new()
            .align_items(Align::Center)
            .push(core_input)
            .push(memory_input)
            .push(video_input)
            .push(apply_button)
            .spacing(20)
            .into()
    }
}

fn main() {
    State::run(Settings::default());
}
