#![windows_subsystem = "windows"]
extern crate iced;
extern crate iced_graphics;
extern crate rand;

use iced::{
    widget::{image, Image, Text},
    Align, Color, Column, Command, Container, Element, Length, Subscription,
};

include!(concat!(env!("OUT_DIR"), "/pepes.rs"));

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
enum Message {
    Ignored,
    MouseMove(Position),
    MouseDown,
    MouseUp,
    Rendered,
}

struct Pepe {
    selected_pepe: usize,

    joke: String,

    changed: bool,

    drag_start_mouse_pos: Position,
    skip_next_mouse_move: bool,
    mouse_pos: Position,
    mouse_down: bool,
    dragging: bool,
}

struct Style {}

impl iced::container::StyleSheet for Style {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(iced::Background::Color(Color{
                a: 0.7,
                ..Color::BLACK
            })),
            ..iced::container::Style::default()
        }
    }
}

const PEPE_SIZE: u32 = 128;
const TEXT_PADDING: u32 = 12;

impl iced_winit::Program for Pepe {
    type Renderer = iced_wgpu::Renderer;
    type Message = Message;

    fn update(&mut self, _: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut col = Column::new()
            .align_items(Align::End)
            .width(Length::Units(PEPE_SIZE as u16))
            .height(Length::Shrink);

        if !self.joke.is_empty() {
            col = col.push(
                Container::new(
                    Text::new(self.joke.clone())
                        .color(Color::WHITE)
                )
                    .width(Length::Fill)
                    .style(Style {})
                    .padding(TEXT_PADDING as u16),
            )
        }

        col.push(
            Container::new(
                Image::new(image::Handle::from_memory(PEPES[self.selected_pepe].into()))
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .align_y(Align::End)
            .align_x(Align::Center)
            .height(Length::Units(PEPE_SIZE as u16)),
        )
        .into()
    }
}

fn next_joke() -> String {
    if rand::random() {
        String::from("Oh god you use spotify? What are you a basic white girl? Need a Trenti iced coffee, 12 pumps vanilla, 6 pumps hazelnut, 4 pumps caramel, 5 pumps skinny mocha, a splash of soy, coffee to the star on the siren's head, ice, double-blended!")
    } else {
        String::from("omg are you a l337 hax0r???? can you teech me how 2 hax and get free roblox")
    }
}

impl iced_winit::application::Application for Pepe {
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                selected_pepe: 0,
                joke: String::default(),
                changed: true,
                skip_next_mouse_move: false,
                drag_start_mouse_pos: Position::default(),
                mouse_pos: Position::default(),
                mouse_down: false,
                dragging: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("clippy but pepe")
    }

    fn update2(
        &mut self,
        window: &iced_winit::winit::window::Window,
        renderer: &Self::Renderer,
        message: Message,
    ) -> Command<Message> {
        use Message::*;

        match message {
            MouseMove(pos) => {
                if !self.dragging && self.mouse_down {
                    self.dragging = true;
                    self.drag_start_mouse_pos = pos;
                } else if self.skip_next_mouse_move {
                    self.skip_next_mouse_move = false;
                } else if self.dragging {
                    let window_pos = window.outer_position().unwrap();
                    window.set_outer_position(iced_winit::winit::dpi::PhysicalPosition::<i32> {
                        x: window_pos.x - self.drag_start_mouse_pos.x as i32 + pos.x as i32,
                        y: window_pos.y - self.drag_start_mouse_pos.y as i32 + pos.y as i32,
                    });
                    self.skip_next_mouse_move = true;
                }

                self.mouse_pos = pos
            }
            MouseDown => {
                self.mouse_down = true;
            }
            MouseUp => {
                self.mouse_down = false;

                if self.dragging {
                    self.dragging = false;
                } else {
                    let prev_pepe = self.selected_pepe;
                    while self.selected_pepe == prev_pepe {
                        self.selected_pepe = rand::random::<usize>() % PEPES.len();
                    }

                    if self.joke.is_empty() {
                        if rand::random::<u8>() < 16 {
                            // approx 1 in 16 chance to show joke
                            self.joke = next_joke();
                            self.changed = true;
                        }
                    } else if rand::random::<u8>() < 32 {
                        // approx 1 in 8 chance to hide joke
                        self.joke = String::default();
                        self.changed = true;
                    }
                }
            }
            Rendered => {
                if self.changed {
                    use iced_native::widget::text::Renderer;
                    let mut height: u32 = 0;
                    if !self.joke.is_empty() {
                        let (_, h) = renderer.measure(
                            self.joke.as_str(),
                            renderer.default_size(),
                            iced_native::Font::default(),
                            iced_native::Size {
                                width: PEPE_SIZE as f32 - 2.0 * TEXT_PADDING as f32,
                                height: 768f32,
                            },
                        );

                        height = h as u32 + TEXT_PADDING * 2;
                    }

                    let window_size = window.inner_size();
                    let new_window_size = iced_winit::winit::dpi::PhysicalSize::<u32> {
                        width: PEPE_SIZE,
                        height: PEPE_SIZE + height,
                    };

                    let window_pos = window.outer_position().unwrap();
                    let new_window_pos = iced_winit::winit::dpi::PhysicalPosition::<i32> {
                        x: window_pos.x,
                        y: i32::max(
                            0,
                            window_pos.y + window_size.height as i32
                                - new_window_size.height as i32,
                        ),
                    };
                    window.set_inner_size(new_window_size);
                    window.set_outer_position(new_window_pos);
                    self.changed = false;
                }
            }
            _ => (),
        };

        Command::none()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(|e| match e {
            iced_native::Event::Mouse(event) => match event {
                iced_native::mouse::Event::ButtonPressed(_) => Message::MouseDown,
                iced_native::mouse::Event::ButtonReleased(_) => Message::MouseUp,
                iced_native::mouse::Event::CursorMoved { x, y } => {
                    Message::MouseMove(Position { x: x, y: y })
                }
                _ => Message::Ignored,
            },
            iced_native::Event::Render => Message::Rendered,
            _ => Message::Ignored,
        })
    }
}

fn main() -> Result<(), iced_winit::Error> {
    let settings = iced_winit::Settings::<()> {
        window: iced_winit::settings::Window {
            decorations: false,
            transparent: true,
            always_on_top: true,
            size: (PEPE_SIZE, 768),
            min_size: Some((PEPE_SIZE, PEPE_SIZE)),
            max_size: Some((PEPE_SIZE, 768)),
            ..iced_winit::settings::Window::default()
        },
        flags: (),
    };

    let renderer_settings = iced_wgpu::Settings {
        antialiasing: Some(iced_wgpu::settings::Antialiasing::MSAAx4),
        default_text_size: 16,
        ..iced_wgpu::Settings::default()
    };

    iced_winit::application::run::<Pepe, iced::executor::Default, iced_wgpu::window::Compositor>(
        settings.into(),
        renderer_settings,
    )
}
