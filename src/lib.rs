use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use tera::{Context, Tera, Value};

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

  pub fn draw_svg(&self, width: usize, height: usize) -> String {

    let mut context = Context::new();
  
    //hardset the padding around the graph
    let padding = 50;
  
    //ensure the viewbox is as per input
    let width = width - padding * 2;
    let height = height - padding * 2;
  
    context.insert("name", &self.name);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("padding", &padding);
  
    Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    
  }
}


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>) -> Graph {
  let mut graph = Graph::new("Example".into(), "#8ff0a4".into());
  for i in 0..xs.len() {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 




/*pub fn vec2str(s: Vec<f64>) {
  return serde_json::to_string(&s).unwrap();
}*/

#[wasm_bindgen]
pub fn get_svg(xstr: &str, ystr: &str) {
  let xs: Vec<f64> = serde_json::from_str(&xstr).unwrap();
  let ys: Vec<f64> = serde_json::from_str(&ystr).unwrap();
  let graph = generate_graph(xs, ys);
  println!("{}", graph.draw_svg(800, 400));
}