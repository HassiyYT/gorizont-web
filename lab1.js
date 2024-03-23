// Import the lodash library
const _ = require('lodash');

// Example array of user objects
const users = [
  { 'user': 'barney', 'active': true },
  { 'user': 'fred',   'active': false },
  { 'user': 'pebbles', 'active': true }
];

// Use lodash's filter function to get all active users
const activeUsers = _.filter(users, { 'active': true });

// Log the result
console.log(activeUsers);
