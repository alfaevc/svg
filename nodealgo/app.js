const { lin_reg, log_reg, glm, svm, kmeans, nb, gmm, dbscan, pca, gp,
        plot_lin_reg, plot_log_reg, plot_glm, plot_nnet, plot_svm, plot_kmeans, plot_nb, plot_gmm, plot_dbscan, plot_pca, plot_gp} = require('../pkg/svg_lib.js');
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
// var model = "gp";


var train_csv = fs.readFileSync("nodealgo/iris.data.csv");
var test_csv = fs.readFileSync("nodealgo/iris.data.csv");
// var out = get_svg(iris_csv, JSON.stringify(centers), width, height, p, title, model);
// console.error(get_svg(iris_csv, width, height, p, title, model));


if (model == "lin_reg") {
    lin_mod = lin_reg(train_csv);
    svg = plot_lin_reg(test_csv, lin_mod, 1);
}
if (model == "log_reg") {
    log_mod = log_reg(train_csv);
    svg = plot_log_reg(test_csv, log_mod, 1);
}
if (model == "nnet") {
    svg = plot_nnet(test_csv);
}
if (model == "nb") {
    nb_mod = nb(train_csv);
    svg = plot_nb(test_csv, nb_mod, 1);
}
if (model == "kmeans") {
    kmeans_mod = kmeans(train_csv, 2);
    svg = plot_kmeans(test_csv, 2, kmeans_mod, 1);
}
if (model == "svm") {
    svm_mod = svm(train_csv);
    svg = plot_svm(test_csv, svm_mod, 1);
}
if (model == "glm") {
    gl_mod = glm(train_csv);
    svg = plot_glm(test_csv, gl_mod, 1);
}
if (model == "gmm") {
    gm = gmm(train_csv, 2);
    svg = plot_gmm(test_csv, 2, gm, 1);
}
if (model == "dbscan") {
    dbscan_mod = dbscan(train_csv);
    svg = plot_dbscan(train_csv, dbscan_mod, 1);
}

if (model == "pca") {
    pca_mod = pca(train_csv);
    svg = plot_pca(test_csv, pca_mod, 1);
}

if (model == "gp") {
    gaussp = gp(train_csv);
    svg = plot_gp(test_csv, gaussp, 1);
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
