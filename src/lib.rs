use wasm_bindgen::prelude::*;
// use std::io::prelude::*;
//use serde::{Serialize, Deserialize};
// use serde_json;
use tera::{Context, Tera};
use std::str::FromStr;
use std::iter::Iterator;
use std::any::type_name;
use std::string::String;
use core::option::Option;

extern crate rusty_machine as rm;
// use rm::prelude::*;
use rm::linalg::Matrix;
use rm::linalg::Vector;
use rm::learning::lin_reg::LinRegressor;
use rm::learning::SupModel;
use rm::learning::k_means::KMeansClassifier;
use rm::learning::UnSupModel;


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
    pub model: String,
    pub size: usize,
    pub points: Vec<Point>,
    pub colour: String,
    pub x_range: f64,
    pub y_range: f64,
    pub x_min: f64,
    pub y_min: f64,
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Graph {
  pub fn new(name: String, model: String, colour: String) -> Self {
      Graph {
          name,
          model,
          size: 0,
          points: Vec::new(),
          colour,
          x_range : 0.,
          y_range : 0.,
          x_min : 0.,
          y_min : 0.,
      }
  }

  pub fn add_point(&mut self, x: f64, y: f64) {
      self.points.push(Point { x, y });
  }

  pub fn draw_svg(&self, width: usize, height: usize, padding: usize) -> String {

    let mut context = Context::new();
    
    // let mut p: Vec<(f64, f64)> = Vec::new();
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut p_vec: Vec<f64> = Vec::new();

    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    
    let path: Vec<(f64, f64)> = self
                              .points
                              .iter()
                              .map(|val|
                                  //x: (val.x / graph.max_x * width as f64) + padding as f64,
                                  //y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
                                  (((val.x-self.x_min) / self.x_range * width as f64) + padding as f64,
                                   ((val.y-self.y_min) / self.y_range * (height as f64 * -1.0)) + (padding + height) as f64)
                                    ).collect();
    /* let path : Vec<Point> = self
              .points
              .iter()
              .map(|val| Point {
                  //x: (val.x / graph.max_x * width as f64) + padding as f64,
                  //y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
                  x: ((val.x-self.x_min) / self.x_range * width as f64) + padding as f64,
                  y: ((val.y-self.y_min) / self.y_range * (height as f64 * -1.0)) + (padding + height) as f64,
              }).collect(); */
            //  .enumerate()
            //  .map(|(i, point)| {
            //      if i == 0 {
            //          format!("M {} {}", point.x, point.y)
            //      } else {
            //          format!("L {} {}", point.x, point.y)
            //      }
            //  })
            //  .collect::<Vec<String>>().join(" ");

    //ensure the viewbox is as per input
    /*for point in path {
      p.push((point.x, point.y));
    }*/

    context.insert("name", &self.name);
    context.insert("model", &self.model);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("padding", &padding);
    context.insert("path", &path);
    context.insert("x_range", &self.x_range);
    context.insert("y_range", &self.y_range);
    context.insert("x_min", &self.x_min);
    context.insert("y_min", &self.y_min);
    context.insert("colour", &self.colour);
    context.insert("lines", &10);

    // println!("graph inputs done!");

    if self.model == "Linear Regression".to_string() {
      let inputs = Matrix::new(xs.len(), 1, xs);
      let targets = Vector::new(ys);
      let mut lin_mod = LinRegressor::default();
      lin_mod.train(&inputs, &targets).unwrap();
      let params : Option<&Vector<f64>> = lin_mod.parameters();
      let mut coefs : Vec<f64> = Vec::new();
      let mut p1 : (f64, f64) = (0.0, 0.0);
      let mut p2 : (f64, f64) = (0.0, 0.0);


      if params.is_some() {
        // println!("{}", params.unwrap().size());
        for i in 0..params.unwrap().size() {
          coefs.push(params.unwrap()[i]);
        }
      }
      if coefs.len() > 0 {
        p1 = (self.x_min, coefs[0] + coefs[1] * self.x_min);
        p2 = (self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range));
        p1 = ((p1.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                  (p1.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);
        p2 = ((p2.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                  (p2.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);

      }
      // println!("{:?}", type_of(p1));
      // println!("{:?}", p2);
      context.insert("point1", &p1);
      context.insert("point2", &p2);


    } else if self.model == "Logistic Regression".to_string() {

    } else if self.model == "Generalized Linear Models".to_string() {

    } else if self.model == "K-Means Clustering".to_string() {
      // println!("We are doing kmeans!");
      let center_num : usize = 3;
      let inputs = Matrix::new(xs.len(), 2, p_vec);
      // println!("{:?}", inputs);
      let mut km = KMeansClassifier::new(center_num);
      println!("We are doing kmeans!");
      km.train(&inputs).unwrap();
      println!("Kmean model trained!");
      let center_mat : &Option<Matrix<f64>> = km.centroids();
      // println!("We are doing kmeans!");
      if center_mat.as_ref().is_some() {
        println!("{:?}", center_mat.as_ref().unwrap());
      }

      let mut centers: Vec<(f64, f64)> = Vec::new();

      /*for i in 0..center_vec.len() {
        if (i % 2) == 1 {
          centers.push((center_vec[i-1], center_vec[i]));
        } 
      }
      let centers = centers
                  .iter()
                  .map(|val| ((val.0-self.x_min) / self.x_range * width as f64 + padding as f64, 
                       (val.1-self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64)).collect();
      */
      context.insert("centers", &centers);
    } else if self.model == "Neural Networks".to_string() {
      
    } else if self.model == "Support Vector Machines".to_string() {

    } else if self.model == "Gaussian Mixture Models".to_string() {

    } else if self.model == "Naive Bayes Classifiers".to_string() {

    } else if self.model == "DBSCAN".to_string() {

    }
  
    Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    
  }
}


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str, model : &str) -> Graph {
  let mut graph = Graph::new(title.into(), model.into(), "#8ff0a4".into());
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
pub fn get_svg(csv_content: &[u8], width: usize, height: usize, padding: usize, title: &str, model: &str) -> String {
  let csv_info: (Vec<f64>, (usize, usize)) = read_data(csv_content);
  let data: Vec<f64> = csv_info.0;
  let dim: (usize, usize) = csv_info.1;
  let mut xs: Vec<f64> = Vec::new();
  let mut ys: Vec<f64> = Vec::new();
  let mut tuples: Vec<(f64, f64)> = Vec::new();
  // let mut centers: Vec<(f64, f64)> = Vec::new();
  // println!("Width is {}", dim.1);
  // let center_vec: Vec<f64> = serde_json::from_str(&center_json).unwrap();

  /* for i in 0..center_vec.len() {
    if (i % 2) == 1 {
      centers.push((center_vec[i-1], center_vec[i]));
    } 
  }*/

  for i in 0..data.len() {
    if (i % 2) == 1 {
      tuples.push((data[i-1], data[i]));
    }
  }
  // assert!(xs.len() == ys.len());
  // tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

  for i in 0..tuples.len() {
    xs.push(tuples[i].0);
    ys.push(tuples[i].1);
  }



  // let permutation = permutation::sort(&xs[..]);
  // let ys = permutation.apply_slice(&ys[..]);
  // let xs = permutation.apply_slice(&xs[..]);
  // println!("Graph is not done!");

  let mut graph = generate_graph(xs, ys, title, model);

  // println!("Graph is done!");

  let width = width - padding * 2;
  let height = height - padding * 2;
  //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
  let x_max = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::max);
  let x_min = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::min);
  let y_max = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::max);
  let y_min = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::min);

  graph.x_min = (x_min-1.0).round();
  graph.y_min = (y_min-1.0).round();

  graph.x_range = (x_max+1.0).round() - graph.x_min;
  graph.y_range = (y_max+1.0).round() - graph.y_min;


   
  
  //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);

  /* let centers = centers
                  .iter()
                  .map(|val| ((val.0-graph.x_min) / graph.x_range * width as f64 + padding as f64, 
                       (val.1-graph.y_min) / graph.y_range * (height as f64 * -1.0) + (padding + height) as f64)).collect();
  */


  let out = graph.draw_svg(width, height, padding);
  // println!("{}", out);
  return out;
}

fn read_data(csv_content: &[u8]) -> (Vec<f64>, (usize, usize)) {
  let v : Vec<u8> = csv_content.to_vec();
  println!("INPUT length is {}", v.len());

  let mut data_reader = csv::Reader::from_reader(csv_content);
  let mut data: Vec<f64> = Vec::new();
  let mut dim: (usize, usize) = (0, 0);
  let mut read_column: bool = false;

  for record in data_reader.records() {
    dim.0 += 1;
    if !read_column {
      for field in record.unwrap().iter() {
          let value = f64::from_str(field);
          data.push(value.unwrap());
          dim.1 += 1;
      }
      read_column = true;
    } else {
      for field in record.unwrap().iter() {
        let value = f64::from_str(field);
        data.push(value.unwrap());
      }
    }
  }
  // println!("{:?}", Matrix::new(2,3, vec![1.5,1.5,1.5,2.5,2.5,2.5]));
  return (data, dim);
}

/* pub fn print_mat() -> {
  // type: rulinalg::matrix::Matrix<f64>
  return Matrix::new(2,3, vec![1.5,1.5,1.5,2.5,2.5,2.5]);
}*/

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}
