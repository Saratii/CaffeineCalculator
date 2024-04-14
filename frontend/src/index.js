import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import ResponseBox from './components/ResponseBox/ResponseBox';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <App>
      <ResponseBox></ResponseBox>
    </App>
  </React.StrictMode>
);