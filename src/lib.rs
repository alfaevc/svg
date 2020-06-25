use wasm_bindgen::prelude::*;
// use std::io::prelude::*;
//use serde::{Serialize, Deserialize};
// use serde_json;
use tera::{Context, Tera};
use std::str::FromStr;
use std::iter::Iterator;
// use std::slice::sort_by;
// use ndarray::{Array2};
// use std::fs::File;
// use std::io::Write;
// use std::clone;
// use std::os::raw::{c_char};
// use std::ffi::{CString};

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub size: usize,
    pub points: Vec<Point>,
    pub colour: String,
    pub max_x: f64,
    pub max_y: f64
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
          size: 0,
          points: Vec::new(),
          colour,
          max_x : 0.,
          max_y : 0.,
      }
  }

  pub fn add_point(&mut self, x: f64, y: f64) {
      self.points.push(Point { x, y });
  }

  pub fn draw_svg(&self, width: usize, height: usize, padding: usize, path: Vec<Point>) -> String {

    let mut context = Context::new();
    
    let mut p: Vec<(f64, f64)> = Vec::new();

    for point in path {
      p.push((point.x, point.y));
    }
    //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
    /*let max_x = self
      .points
      .iter()
      .map(|point| point.x)
      .fold(0. / 0., f64::max);

    //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);
    let max_y = self
      .points
      .iter()
      .map(|point| point.y)
      .fold(0. / 0., f64::max);
      //hardset the padding around the graph
    */
    // let c_str = CString::new(file).unwrap();
    // let filename: *const c_char = c_str.as_ptr() as *const c_char;
    // const filename: &str = file.clone();

    //ensure the viewbox is as per input
  
    context.insert("name", &self.name);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("padding", &padding);
    context.insert("path", &p);
    context.insert("max_x", &self.max_x);
    context.insert("max_y", &self.max_y);
    context.insert("colour", &self.colour);
    context.insert("lines", &5);
  
    Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    
  }
}


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str) -> Graph {
  let mut graph = Graph::new(title.into(), "#8ff0a4".into());
  graph.size = xs.len();
  for i in 0..graph.size {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 




/*pub fn vec2str(s: Vec<f64>) {
  return serde_json::to_string(&s).unwrap();
}*/

#[wasm_bindgen]
pub fn get_svg(csv_content: &[u8], width: usize, height: usize, padding: usize, title: &str) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let mut xs: Vec<f64> = Vec::new();
  let mut ys: Vec<f64> = Vec::new();
  let mut tuples: Vec<(f64, f64)> = Vec::new();

  for i in 0..data.len() {
    if (i % 2) == 1 {
      tuples.push((data[i-1], data[i]));
    }
  }
  // assert!(xs.len() == ys.len());
  tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

  for i in 0..tuples.len() {
    xs.push(tuples[i].0);
    ys.push(tuples[i].1);
  }



  // let permutation = permutation::sort(&xs[..]);
  // let ys = permutation.apply_slice(&ys[..]);
  // let xs = permutation.apply_slice(&xs[..]);

  let mut graph = generate_graph(xs, ys, title);



  let width = width - padding * 2;
  let height = height - padding * 2;
  //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
  let x_max_bound = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::max);
  let x_min_bound = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::min);
  let y_max_bound = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::max);
  let y_min_bound = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::min);

  if x_max_bound < -x_min_bound {
    graph.max_x = -x_min_bound;
  } else {
    graph.max_x = x_max_bound;
  }

  if y_max_bound < -y_min_bound {
    graph.max_y = -y_min_bound;
  } else {
    graph.max_y = y_max_bound;
  }
   
  
  //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);

  let path = graph
              .points
              .iter()
              .map(|val| Point {
                  //x: (val.x / graph.max_x * width as f64) + padding as f64,
                  //y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
                  x: ((val.x+graph.max_x) / (2.0*graph.max_x) * width as f64) + padding as f64,
                  y: ((val.y+graph.max_y) / (2.0*graph.max_y) * (height as f64 * -1.0)) + (padding + height) as f64,
              }).collect();
            //  .enumerate()
            //  .map(|(i, point)| {
            //      if i == 0 {
            //          format!("M {} {}", point.x, point.y)
            //      } else {
            //          format!("L {} {}", point.x, point.y)
            //      }
            //  })
            //  .collect::<Vec<String>>().join(" ");

  let out = graph.draw_svg(width, height, padding, path);
  // println!("{}", out);
  return out;
}

fn read_data(csv_content: &[u8]) -> Vec<f64> {
  let v : Vec<u8> = csv_content.to_vec();
  println!("INPUT length is {}", v.len());

  let mut data_reader = csv::Reader::from_reader(csv_content);
  let mut data: Vec<f64> = Vec::new();
  for record in data_reader.records() {
      for field in record.unwrap().iter() {
          let value = f64::from_str(field);
          data.push(value.unwrap());
      }
  }
  return data;
}
