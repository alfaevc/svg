const express = require('express');
const { get_svg } = require('../pkg/svg_lib.js');
const fs = require('fs');
var http = require('http');
var router = express.Router();

const app = express();
const port = 8080;
app.use(express.static('public'));
app.use(express.urlencoded({ extended: false }));
/*
var bodyParser = require('body-parser')
app.use(bodyParser.urlencoded({
  extended: true
})); 
*/

var width = 800;
var height = 400;
var p = 50;

function checknull(arr) {
    var i;
    if(arr.length < 1 || arr == undefined){
        return true;
    }
    for (i = 0; i < arr.length; i++) {
        if (arr[i] == null) {
            return true;
        }
    }
    return false;
}

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/submit', function (req, res) {
    var re = ",";
    var xs = req.body.xcords.split(re).map(Number);
    if (checknull(xs))
    var ys = req.body.ycords.split(re).map(Number);

    res.send(submit([a, b, c]))
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))