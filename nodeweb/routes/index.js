const { get_svg } = require('../../pkg/svg_lib.js');
const fs = require('fs');
var http = require('http');
var express = require('express');
var router = express.Router();

// var x = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.];
// var y = [10., 3., 17., 20., 25., 23., 30., 38.2, 32.5, 40.8, 35.7, 21.7];
var width = 800;
var height = 400;
var p = 50;
// var title = "SVG";
// var out = get_svg(JSON.stringify(x), JSON.stringify(y), width, height, p, title)

/*fs.writeFile('src/out.svg', out, (err) => {
    if (err) throw err;
    console.log('The file has been saved!');
});*/

function checknull(arr, axis) {
  var i;
  if(arr.length < 1 || arr == undefined){
    return true;
  }
  for (i = 0; i < arr.length; i++) {
    if (isNaN(arr[i])) {
      return true;
    }
    if (axis == 1) { // if x axis then check sorted
      if (i > 0 && arr[i] <= arr[i-1]) {
        return true;
      }
    }
  }
  return false;
}

function onRequest(req, res) {
  res.writeHead(200, {'Content-Type': 'text/xml'});
  // res.write(out);
  fs.readFile("../../src/out.svg", null, function(error, data) {
      if (error) {
          res.writeHead(404);
          res.write("File not found!");
      } else {
          res.write(data);
      }
      res.end();
  })
  // res.end();
}

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'SVG', success: req.session.success, errors: req.session.errors, graph: req.session.graph});
  req.session.errors = null;
  // req.session.graph = null;
});

router.post('/submit', function(req, res, next) {
  var re = ",";
  var xs = req.body.xcords.split(re).map(Number);
  var ys = req.body.ycords.split(re).map(Number);
  console.log(xs);
  // console.log(xs[0]+xs[1]);
  req.check("xcords", "Invalid x coordinates").notEmpty().custom(value => !checknull(xs, 1));
  // req.check("xcords", "Invalid x coordinates").isEmail();
  req.check("ycords", "Invalid y coordinates").notEmpty().custom(value => !checknull(ys, 2) && (xs.length === ys.length));
  // console.log(2);
  req.check("svgtitle", "Title can't be empty").notEmpty();
  // console.log(3);
  errors = req.validationErrors();
  // console.log(4);
  // var errors = checknull(xs) || checknull(ys) || (xs.length != ys.length);
  if (errors) {
    req.session.errors = errors;
    req.session.success = false;
    req.session.graph = "";
  } else {
    req.session.success = true;
    var out = get_svg(JSON.stringify(xs), JSON.stringify(ys), width, height, p, req.body.svgtitle);
    // var svg = out.replace("/\<\?xml(.+?)\?\>/g", "");

    fs.writeFile('../src/out.svg', out, (err) => {
      if (err) throw err;
      console.log('The file has been saved!');
    });
    svg = out.replace(/\<\?xml.+\?\>/g, '');
    console.log(svg);
    req.session.graph = out;
    // http.createServer(onRequest).listen(3000);
  }
  res.redirect('/');
});


// http.createServer(onRequest).listen(8080)

/* GET users listing. */

/*router.get('/test/:id', function(req, res, next) {
  res.render('test', {output: req.params.id});
  //res.send('respond with a resource');
});

router.post('/test/submit', function(req, res, next) {
  var id = req.body.id
  res.redirect('/test/' + id);
});*/


//router.get('/users/svg', function(req, res, next) {
//  res.send('svg');
//});

module.exports = router;
