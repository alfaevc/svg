const { get_svg } = require('../pkg/svg_lib.js');
const fs = require('fs');

var http = require('http')
// var x = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.];
// var y = [10., 3., 17., 20., 25., 23., 30., 38.2, 32.5, 40.8, 35.7, 21.7];
var width = 800;
var height = 400;
var p = 50;
var title = "SVG";
var model = "K-Means Clustering";

// var centers = [3.44, 0.24, 3.04, 2.05, 2.71, 1.31];
var iris_csv = fs.readFileSync("nodealgo/iris.data.csv");
// var out = get_svg(iris_csv, JSON.stringify(centers), width, height, p, title, model);
var out = get_svg(iris_csv, width, height, p, title, model);

fs.writeFile('src/out.svg', out, (err) => {
    if (err) throw err;
    console.log('The file has been saved!');
});
// console.log(out);
// resize_file(JSON.stringify([dim, 'Vincent_van_Gogh_368.jpg', `test.png`]));

// app.get('/', (req, res) => res.send(out))

//app.listen(port, () => console.log(`Example app listening at http://localhost:${port}`))

function onRequest(req, res) {
    res.writeHead(200, {'Content-Type': 'text/xml'});
    // res.write(out);
    fs.readFile("src/out.svg", null, function(error, data) {
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

http.createServer(onRequest).listen(8080)
