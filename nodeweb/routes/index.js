var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'Cool', condition: false, anyArray: [1,2,3] });
});

/* GET users listing. */
router.get('/test/:id', function(req, res, next) {
  res.render('test', {output: req.params.id});
  res.send('respond with a resource');
});

router.post('/test/submit', function(req, res, next) {
  var id = req.body.id
  res.redirect('/test/' + id);
});


//router.get('/users/svg', function(req, res, next) {
//  res.send('svg');
//});

module.exports = router;
