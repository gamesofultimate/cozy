import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from 'components/App';
import { GameProvider } from 'data/game';

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);

root.render(
  <GameProvider>
    <App />
  </GameProvider>
);
