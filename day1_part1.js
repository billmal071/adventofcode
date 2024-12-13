const fs = require('fs');

function total_distance_btw_list() {
  fs.readFile('day1.txt', 'utf8', (err, data) => {
    if (err) {
      console.error('Error reading file:', err);
      return;
    }
    console.log('File content:', data);
  });
}

total_distance_btw_list()