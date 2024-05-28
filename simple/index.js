// A express server to run a system command
const express = require('express');
const {exec} = require('child_process');

const app = express();
const port = 3000;

// Middleware to parse JSON bodies
app.use(express.json());

// Route to run a system command
app.post('/run-command', (req, res) => {
    const {command} = req.body;

    if (!command) {
        return res.status(400).send('Command is required');
    }

    // Execute the command
    exec(command, (error, stdout, stderr) => {
        if (error) {
            console.error(`Error executing command: ${error.message}`);
            return res.status(500).send(`Error: ${error.message}`);
        }

        if (stderr) {
            console.error(`Command stderr: ${stderr}`);
            return res.status(500).send(`Error: ${stderr}`);
        }

        console.log(`Command stdout: ${stdout}`);
        res.send(`Output: ${stdout}`);
    });
});

app.listen(port, () => {
    console.log(`Server is running on http://localhost:${port}`);
});
