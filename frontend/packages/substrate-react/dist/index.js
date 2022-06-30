
'use strict'

if (process.env.NODE_ENV === 'production') {
  module.exports = require('./substrate-react.cjs.production.min.js')
} else {
  module.exports = require('./substrate-react.cjs.development.js')
}
