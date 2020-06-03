use wasm_bindgen::prelude::*;
//use serde::{Serialize, Deserialize};
use serde_json;
use tera::{Context, Tera};
// use std::clone;
// use std::os::raw::{c_char};
// use std::ffi::{CString};

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub points: Vec<Point>,
    pub colour: String
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Graph {
  pub fn new(name: String, colour: String) -> Self {
      Graph {
          name,
          points: Vec::new(),
          colour,
      }
  }

  pub fn add_point(&mut self, x: f64, y: f64) {
      self.points.push(Point { x, y });
  }

  pub fn draw_svg(&self, width: usize, height: usize, padding: usize, path: String, file: &str) -> String {

    let mut context = Context::new();

  
    //hardset the padding around the graph

    // let c_str = CString::new(file).unwrap();
    // let filename: *const c_char = c_str.as_ptr() as *const c_char;
    // const filename: &str = file.clone();

    //ensure the viewbox is as per input
  
    context.insert("name", &self.name);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("padding", &padding);
    context.insert("path", &path);
    context.insert("colour", &self.colour);
    context.insert("lines", &5);
  
    Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    
  }
}


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>) -> Graph {
  let mut graph = Graph::new("SVG".into(), "#8ff0a4".into());
  for i in 0..xs.len() {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 




/*pub fn vec2str(s: Vec<f64>) {
  return serde_json::to_string(&s).unwrap();
}*/

#[wasm_bindgen]
pub fn get_svg(xstr: &str, ystr: &str, width: usize, height: usize, padding: usize, file: &str) -> String {
  let xs: Vec<f64> = serde_json::from_str(&xstr).unwrap();
  let ys: Vec<f64> = serde_json::from_str(&ystr).unwrap();
  let graph = generate_graph(xs, ys);
  let width = width - padding * 2;
  let height = height - padding * 2;
  //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
  let max_x = graph
    .points
    .iter()
    .map(|point| point.x)
    .fold(0. / 0., f64::max);
  
  //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);
  let max_y = graph
    .points
    .iter()
    .map(|point| point.y)
    .fold(0. / 0., f64::max);

  let path = graph
              .points
              .iter()
              .map(|val| Point {
                  x: (val.x / max_x * width as f64) + padding as f64,
                  y: (val.y / max_y * (height as f64 * -1.0)) + (padding + height) as f64,
              })
              .enumerate()
              .map(|(i, point)| {
                  if i == 0 {
                      format!("M {} {}", point.x, point.y)
                  } else {
                      format!("L {} {}", point.x, point.y)
                  }
              })
              .collect::<Vec<String>>().join(" ");
  
  let out = graph.draw_svg(width, height, padding, path, file);
  println!("{}", out);
  return out;
}
