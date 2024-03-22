// Import the lodash library
const _ = require('lodash');

// An array of numbers
const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Use the _.filter function to get all even numbers in the array
const evenNumbers = _.filter(numbers, (n) => n % 2 === 0);

console.log(evenNumbers); // Output: [2, 4, 6, 8, 10]
