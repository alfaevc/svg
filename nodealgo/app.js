const { lin_reg, log_reg, glm, svm, kmeans, nb, gmm, dbscan, pca, 
        plot_lin_reg, plot_log_reg, plot_glm, plot_nnet, plot_svm, plot_kmeans, plot_nb, plot_gmm, plot_dbscan, plot_pca } = require('../pkg/svg_lib.js');
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
var model = "dbscan";
// var model = "pca";


var iris_csv = fs.readFileSync("nodealgo/iris.data.csv");
// var out = get_svg(iris_csv, JSON.stringify(centers), width, height, p, title, model);
// console.error(get_svg(iris_csv, width, height, p, title, model));


if (model == "lin_reg") {
    lin_mod = lin_reg(iris_csv);
    svg = plot_lin_reg(iris_csv, lin_mod, 1);
}
if (model == "log_reg") {
    log_mod = log_reg(iris_csv);
    svg = plot_log_reg(iris_csv, log_mod, 1);
}
if (model == "nnet") {
    svg = plot_nnet(iris_csv);
}
if (model == "nb") {
    nb_mod = log_reg(iris_csv);
    svg = plot_nb(iris_csv, nb_mod, 1);
}
if (model == "kmeans") {
    kmeans_mod = kmeans(iris_csv, 2);
    svg = plot_kmeans(iris_csv, 2, kmeans_mod, 1);
}
if (model == "svm") {
    svm_mod = svm(iris_csv);
    svg = plot_svm(iris_csv, svm_mod, 1);
}
if (model == "glm") {
    gl_mod = glm(iris_csv);
    svg = plot_glm(iris_csv, gl_mod, 1);
}
if (model == "gmm") {
    gm = gmm(iris_csv, 2);
    svg = plot_gmm(iris_csv, 2, gm, 1);
}
if (model == "dbscan") {
    dbscan_mod = dbscan(iris_csv);
    svg = plot_dbscan(iris_csv, dbscan_mod, 1);
}

if (model == "pca") {
    pca_mod = pca(iris_csv);
    svg = plot_pca(iris_csv, pca_mod, 1);
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
