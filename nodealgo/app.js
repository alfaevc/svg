const { plot_lin_reg, plot_log_reg, plot_glm, plot_nnet, plot_svm, plot_kmeans, plot_nb, plot_gmm, plot_dbscan, plot_pca } = require('../pkg/svg_lib.js');
const fs = require('fs');

var http = require('http')
// var width = 800;
// var height = 400;
// var p = 50;
// var title = "SVG";
// Weird behavior for svm
// var model = "lin_reg";
// var model = "log_reg";
// var model = "glm";
// var model = "nnet";
// var model = "svm";
// var model = "kmeans";
// var model = "nb";
// var model = "gmm";
// var model = "dbscan";
var model = "pca";


var iris_csv = fs.readFileSync("nodealgo/iris.data.csv");
// var out = get_svg(iris_csv, JSON.stringify(centers), width, height, p, title, model);
// console.error(get_svg(iris_csv, width, height, p, title, model));

if (model == "lin_reg") {
    svg = plot_lin_reg(iris_csv);
}
if (model == "log_reg") {
    svg = plot_log_reg(iris_csv);
}
if (model == "nnet") {
    svg = plot_nnet(iris_csv);
}
if (model == "nb") {
    svg = plot_nb(iris_csv);
}
if (model == "kmeans") {
    svg = plot_kmeans(iris_csv, 2);
}
if (model == "svm") {
    svg = plot_svm(iris_csv);
}
if (model == "glm") {
    svg = plot_glm(iris_csv);
}
if (model == "gmm") {
    svg = plot_gmm(iris_csv);
}
if (model == "dbscan") {
    svg = plot_dbscan(iris_csv);
}

if (model == "pca") {
    svg = plot_pca(iris_csv);
}


fs.writeFile('src/out.svg', svg, (err) => {
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
