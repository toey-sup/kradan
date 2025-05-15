use iced::advanced::graphics::geometry::Frame;
use iced::widget::canvas;
use iced::widget::canvas::{Group, Stroke};
use iced::{Color, Point, Rectangle, Renderer};

const MAX_CACHE_SIZE: usize = 200;

#[derive(Debug)]
struct LineSegment {
  points: Vec<Point>,
  caches: canvas::Cache,
}

impl LineSegment {
  pub fn with_group(group: Group) -> Self {
    LineSegment {
      points: Vec::new(),
      caches: canvas::Cache::with_group(group),
    }
  }
}

#[derive(Debug)]
pub struct Line {
  pub points: Vec<Point>,
  segments: Vec<LineSegment>,
  cache_group: Group,
}

impl Line {
  pub fn new() -> Self {
    let group = Group::unique();
    Line {
      segments: vec![LineSegment::with_group(group)],
      cache_group: group,
      points: Vec::new(),
    }
  }

  pub fn add_point(&mut self, point: Point) {
    self.points.push(point);
    let last_segment_index: usize = self.segments.len() - 1;
    if self.segments[last_segment_index].points.len() >= MAX_CACHE_SIZE {
      let mut new_segment = LineSegment::with_group(self.cache_group);
      new_segment.points.push(point);
      self.segments.push(new_segment);
    } else {
      self.segments[last_segment_index].points.push(point);
      self.segments[last_segment_index].caches.clear();
    }
  }

  pub fn draw_from_points(frame: &mut Frame<Renderer>, points: &Vec<Point>) {
    if points.len() == 1 {
      let circle = canvas::Path::circle(points[0], 10.00);
      frame.fill(&circle, Color::BLACK);
    } else {
      for i in 1..points.len() {
        let from = points[i - 1];
        let to = points[i];
        let line = canvas::Path::line(from, to);
        frame.stroke(
          &line,
          Stroke::default()
            .with_width(20.00)
            .with_color(Color::BLACK)
            .with_line_join(canvas::LineJoin::Round)
            .with_line_cap(canvas::LineCap::Round),
        );
        frame.fill(&line, Color::BLACK);
      }
    }
  }

  pub fn draw(&self, renderer: &Renderer, bounds: Rectangle) -> Vec<canvas::Geometry> {
    self
      .segments
      .iter()
      .map(|segment| {
        segment.caches.draw(renderer, bounds.size(), |frame| {
          Line::draw_from_points(frame, &segment.points);
        })
      })
      .collect()
  }
}
