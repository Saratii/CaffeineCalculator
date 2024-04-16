import { useState } from 'react';
import './App.css';
import React from 'react';
import Graph from './components/Graph/Graph';

function App() {
  const [response_boxes, set_response_boxes] = useState([]);
  const [graph_points, set_graph_points] = useState([]);
  const [graph_funct, set_graph_funct] = useState([]);
  const [user_inputs, set_user_inputs] = useState([]);
  const [user_input_index, set_user_input_index] = useState([]);
  const response_list = response_boxes.map(response_box =>
      <pre type="text" className="output_text" key={response_box.key}>{response_box.message}</pre>
  );

    function submit(event){
        if (event.keyCode === 13) {
          sendText();
          set_user_input_index(user_inputs.length+1)
        }

        if (event.keyCode === 38 && user_inputs.length > 0 && user_input_index > 0) {
          set_user_input_index(i => i - 1);
          document.getElementById("command-input").value = user_inputs[user_input_index - 1];
        }
        
        if (event.keyCode === 40 && user_inputs.length > 0 && user_input_index < user_inputs.length - 1) {
          set_user_input_index(i => i + 1);
          document.getElementById("command-input").value = user_inputs[user_input_index + 1];
        }
     }

    function createAnswerBox(data) {
        if (data.message.indexOf("},{") > -1) {
            set_graph_points(JSON.parse("[" + data.message + "]"));
            set_graph_funct(document.getElementById("command-input").value.replace(/\bgraph\b|\(|\)/g, ''))
        } else {
            data.key = response_boxes.length;
            set_response_boxes(response_boxes.concat(data));
            document.getElementById("command-input").value = "";
            scrollToBottom();
        }
    }

    function scrollToBottom() {
        var cl = document.getElementById("command-line");
        cl.scrollTop = cl.scrollHeight;
    }

    function sendText() {
        var inputText = document.getElementById("command-input").value;
        set_user_inputs(prevUserInputs => [...prevUserInputs, inputText]);
        fetch('http://127.0.0.1:8080/calculate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ text: inputText })
        })
        .then(response => {
            if (!response.ok) {
                throw new Error('Something went wrong');
            }
            return response.json();
        })
        .then(data => {
            console.log("Received response:", data);
            createAnswerBox(data);
        })
        .catch(error => {
            console.error('Error:', error);
        });
    }

    return (
        <main className="container">
            <div className="top-screen">
                <div className='logo'>CaffeineCalculator</div>
                <div className='help-id'>Command 'help' for help</div>
            </div>
            <div className="blackbox">
                <div className="command-line" id="command-line">
                    {
                        response_list
                    }
                    <span className="blinking">&gt;</span>
                    <input type="text" id="command-input" autoFocus={true} autoCorrect='off' spellCheck='false' onBlur={({ target }) => target.focus()} autoComplete="off" className="input_text" onKeyDown={submit}></input>
                </div>
                <Graph
                    funct={graph_funct}
                    minX={-10}
                    maxX={10}
                    minY={-10}
                    maxY={10}
                    dataPoints={graph_points}>
                </Graph>
            </div>
        </main>
    )
}

export default App;