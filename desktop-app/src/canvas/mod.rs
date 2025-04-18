use iced::widget::canvas;
use iced::widget::canvas::Stroke;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::mouse_area;
use iced::Background;
use iced::{mouse, Theme};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Task};

#[derive(Default)]
pub struct Canvas {
  main_program: MainCanvasProgram,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  MousePress,
  MouseRelease,
  MouseMove(Point),
}

impl Canvas {
  pub fn title(&self) -> String {
    String::from("Dao Kum")
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::MouseMove(point) => {
        self.main_program.handle_mouse_move(point);
        Task::none()
      }
      Message::MousePress => {
        self.main_program.set_is_drawing(true);
        Task::none()
      }
      Message::MouseRelease => {
        self.main_program.set_is_drawing(false);
        Task::none()
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    container(
      mouse_area(
        canvas(&self.main_program)
          .width(Length::Fill)
          .height(Length::Fill),
      )
      .on_release(Message::MouseRelease)
      .on_move(Message::MouseMove)
      .on_exit(Message::MouseRelease)
      .on_press(Message::MousePress),
    )
    .style(|_| Style {
      background: Some(Background::Color(Color::from_rgb(240.0, 240.0, 240.0))),
      ..Default::default()
    })
    .into()
  }
}

#[derive(Debug)]
struct MainCanvasProgram {
  cache: canvas::Cache,
  is_drawing: bool,
  points_set: Vec<Vec<Point>>,
}

impl Default for MainCanvasProgram {
  fn default() -> Self {
    MainCanvasProgram {
      cache: canvas::Cache::default(),
      is_drawing: false,
      points_set: vec![Vec::new()],
    }
  }
}

impl MainCanvasProgram {
  pub fn set_is_drawing(&mut self, value: bool) {
    if self
      .points_set
      .last()
      .map(|last| last.len())
      .unwrap_or(0 as usize)
      != 0
    {
      self.points_set.push(Vec::new());
    }
    self.is_drawing = value
  }

  pub fn handle_mouse_move(&mut self, point: Point) {
    if self.is_drawing {
      let last_index = self.points_set.len() - 1;
      self.points_set[last_index].push(point);
      self.cache.clear();
    }
  }
}

impl<Message> canvas::Program<Message> for MainCanvasProgram {
  type State = ();

  fn draw(
    &self,
    _state: &Self::State,
    renderer: &Renderer,
    _theme: &Theme,
    bounds: Rectangle,
    _cursor: mouse::Cursor,
  ) -> Vec<canvas::Geometry> {
    let picture = self.cache.draw(renderer, bounds.size(), |frame| {
      for set in &self.points_set {
        for i in 1..set.len() {
          let from = set[i - 1];
          let to = set[i];
          // let start_circle = canvas::Path::circle(from, 10.00);
          let line = canvas::Path::line(from, to);
          // let end_circle = canvas::Path::circle(from, 10.00);
          frame.stroke(
            &line,
            Stroke::default().with_width(20.00).with_color(Color::BLACK),
          );
          // frame.fill(&start_circle, Color::BLACK);
          frame.fill(&line, Color::BLACK);
          // frame.fill(&end_circle, Color::BLACK);
        }
      }
    });
    vec![picture]
  }
}
