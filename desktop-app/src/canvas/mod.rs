use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::container;
use iced::widget::container::Style;
use iced::Background;
use iced::{mouse, Theme};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Task};
mod line;
use line::Line;

#[derive(Default)]
pub struct Canvas {
  main_program: MainCanvasProgram,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  MouseClick(Point),
  MouseMove(Point),
  MouseUp,
  MouseLeave,
}

impl Canvas {
  pub fn title(&self) -> String {
    String::from("Dao Kum")
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::MouseClick(point) => {
        self.main_program.is_drawing = true;
        self.main_program.add_point(point);
        Task::done(Message::MouseClick(point))
      }
      Message::MouseLeave => {
        self.main_program.is_drawing = false;
        self.main_program.complete_stroke();
        Task::done(Message::MouseLeave)
      }
      Message::MouseMove(point) => {
        if self.main_program.is_drawing {
          self.main_program.add_point(point);
        }
        Task::done(Message::MouseMove(point))
      }
      Message::MouseUp => {
        self.main_program.is_drawing = false;
        self.main_program.complete_stroke();
        Task::done(Message::MouseUp)
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    container(
      canvas(&self.main_program)
        .width(Length::Fill)
        .height(Length::Fill),
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
  main_cache: canvas::Cache,
  points_set: Vec<Vec<Point>>,
  current_stroke: Line,
  is_drawing: bool,
}

impl Default for MainCanvasProgram {
  fn default() -> Self {
    MainCanvasProgram {
      main_cache: canvas::Cache::new(),
      points_set: Vec::new(),
      current_stroke: Line::new(),
      is_drawing: false,
    }
  }
}

impl MainCanvasProgram {
  pub fn add_point(&mut self, point: Point) {
    self.current_stroke.add_point(point);
  }

  pub fn complete_stroke(&mut self) {
    self.points_set.push(self.current_stroke.points.clone());
    self.current_stroke = Line::new();
    self.main_cache.clear();
  }
}

impl canvas::Program<Message> for MainCanvasProgram {
  type State = ();

  fn update(
    &self,
    _state: &mut Self::State,
    event: Event,
    bounds: Rectangle,
    cursor: mouse::Cursor,
  ) -> (event::Status, Option<Message>) {
    let Some(cursor_position) = cursor.position_in(bounds) else {
      return (event::Status::Ignored, None);
    };

    match event {
      Event::Mouse(mouse_event) => {
        let message = match mouse_event {
          mouse::Event::ButtonPressed(mouse::Button::Left) => {
            Option::Some(Message::MouseClick(cursor_position))
          }
          mouse::Event::CursorMoved { position } => Option::Some(Message::MouseMove(position)),
          mouse::Event::ButtonReleased(mouse::Button::Left) => Option::Some(Message::MouseUp),
          mouse::Event::CursorLeft => Option::Some(Message::MouseLeave),
          _ => None,
        };
        (event::Status::Captured, message)
      }
      _ => (event::Status::Ignored, None),
    }
  }

  fn draw(
    &self,
    _state: &Self::State,
    renderer: &Renderer,
    _theme: &Theme,
    bounds: Rectangle,
    _cursor: mouse::Cursor,
  ) -> Vec<canvas::Geometry> {
    let main_geometry = self.main_cache.draw(renderer, bounds.size(), |frame| {
      for points in &self.points_set {
        Line::draw_from_points(frame, points);
      }
    });
    let mut current_stroke_geometry = self.current_stroke.draw(renderer, bounds);
    current_stroke_geometry.push(main_geometry);
    return current_stroke_geometry;
  }

  fn mouse_interaction(
    &self,
    _state: &Self::State,
    _bounds: Rectangle,
    _cursor: iced::advanced::mouse::Cursor,
  ) -> iced::advanced::mouse::Interaction {
    iced::advanced::mouse::Interaction::default()
  }
}
