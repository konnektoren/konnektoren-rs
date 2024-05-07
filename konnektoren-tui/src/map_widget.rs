use konnektoren_core::game::GamePath;
use ratatui::{
    prelude::*,
    widgets::{canvas::Line, canvas::*, Block},
};

pub struct MapWidget<'a> {
    path: &'a GamePath,
}

impl MapWidget<'_> {
    pub fn calculate_bounds(challenges: &[(String, f64, f64)]) -> ([f64; 2], [f64; 2]) {
        let x_min = challenges
            .iter()
            .map(|(_, x, _)| *x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let x_max = challenges
            .iter()
            .map(|(_, x, _)| *x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let y_min = challenges
            .iter()
            .map(|(_, _, y)| *y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let y_max = challenges
            .iter()
            .map(|(_, _, y)| *y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        ([x_min - 10.0, x_max + 10.0], [y_min - 10.0, y_max + 10.0])
    }
}

impl<'a> MapWidget<'a> {
    pub fn new(path: &'a GamePath) -> Self {
        MapWidget { path }
    }

    fn process_challenges(&self) -> Vec<(String, f64, f64)> {
        self.path
            .challenges
            .iter()
            .map(|challenge| {
                let (x, y) = challenge.position.unwrap_or((0, 0));
                (challenge.name.to_string(), x as f64 * 10.0, y as f64 * 10.0)
            })
            .collect()
    }

    fn draw_map(
        &self,
        title: &str,
        challenges: &[(String, f64, f64)],
        x_bounds: [f64; 2],
        y_bounds: [f64; 2],
        area: Rect,
        buf: &mut Buffer,
    ) {
        let canvas = Canvas::default()
            .block(Block::bordered().title(title))
            .marker(Marker::Braille)
            .paint(|ctx| {
                let mut last: Option<(f64, f64)> = None;
                for (name, x, y) in challenges {
                    ctx.draw(&Rectangle {
                        x: *x,
                        y: *y,
                        width: 1.0,
                        height: 1.0,
                        color: Color::Yellow,
                    });
                    ctx.print(*x, *y, format!("{}", name).yellow());
                    if let Some((x1, y1)) = last {
                        ctx.draw(&Line {
                            x1,
                            y1,
                            x2: *x,
                            y2: *y,
                            color: Color::Yellow,
                        });
                    }
                    last = Some((*x, *y));
                }
            })
            .x_bounds(x_bounds)
            .y_bounds(y_bounds);

        canvas.render(area, buf);
    }
}

impl<'a> Widget for MapWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let canvas = Canvas::default()
            .block(Block::bordered().title("World"))
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);

        canvas.render(area, buf);

        let challenges = self.process_challenges();
        let (x_bounds, y_bounds) = Self::calculate_bounds(&challenges);
        self.draw_map("Challenges", &challenges, x_bounds, y_bounds, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_boundaries() {
        let challenges = vec![
            ("Challenge 1".to_string(), 0.0, 0.0),
            ("Challenge 2".to_string(), 10.0, 10.0),
            ("Challenge 3".to_string(), -10.0, -10.0),
        ];

        let bounds = MapWidget::calculate_bounds(&challenges);
        assert_eq!(bounds, ([-20.0, 20.0], [-20.0, 20.0]));
    }
}
