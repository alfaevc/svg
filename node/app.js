const { get_svg } = require('../pkg/svg_lib.js');

var x = [10., 5.];
var y = [2., 8.];

console.log(get_svg(JSON.stringify(x), JSON.stringify(y)));
// resize_file(JSON.stringify([dim, 'Vincent_van_Gogh_368.jpg', `test.png`]));