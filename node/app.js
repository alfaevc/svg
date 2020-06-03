const { get_svg } = require('../pkg/svg_lib.js');

var x = [1., 2., 3., 4., 5.];
var y = [1., 3., 2.5, 6., 3.];
var width = 800;
var height = 400;
var p = 50;

console.log(get_svg(JSON.stringify(x), JSON.stringify(y), width, height, p, "graph.svg"));
// resize_file(JSON.stringify([dim, 'Vincent_van_Gogh_368.jpg', `test.png`]));
