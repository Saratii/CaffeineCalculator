import { useState } from 'react';
import './App.css';
import React, { Component } from 'react';
import Graph from './components/Graph/Graph';

function App() {
  const [response_boxes, set_response_boxes] = useState([]);
  
  const response_list = response_boxes.map(response_box => 
    <div type="text" className="output_text" key={response_box.key}>{response_box.message}</div>
  );

  const dataPoints = [{ x: 0.0, y: 0.0 }];

  function submit(event){
    if (event.keyCode === 13) {
      sendText();
    }
  }

  function createAnswerBox(data) {
    data.key=response_boxes.length;
    set_response_boxes(response_boxes.concat(data));
    document.getElementById("command-input").value = "";
    scrollToBottom()
  }

  function scrollToBottom() {
    var cl = document.getElementById("command-line");
    cl.scrollTop = cl.scrollHeight;
    console.log(cl.scrollHeight)
  }

  function sendText() {
    var inputText = document.getElementById("command-input").value;
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
        <div className='logo'>ToBeDecided</div>
        <div className='help-id'>Command 'help' for help</div>
      </div>
      <div className="command-line" id="command-line">
        <div>
        <Graph className="graph" dataPoints={dataPoints} />
      </div>
      {
        response_list
      }
        <span className="blinking">&gt;</span>
          <input type="text" id="command-input" autoFocus={true} autoCorrect='off' spellCheck='false' onBlur={({ target }) => target.focus()} autoComplete="off" className="input_text" onKeyDown={submit}></input>
      </div>
    
    </main>
  )
}

export default App;


