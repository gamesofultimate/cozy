import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';

import MainPage from 'components/MainPage';
import MomentPage from 'components/MomentPage';
import ResultsPage from 'components/ResultsPage';
import LoginPage from 'components/LoginPage';
import AcceptInvitationPage from 'components/AcceptInvitationPage';

//import {EnsureInvite} from 'components/EnsureAuth';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/accept-invitation/:invitation_token" element={<AcceptInvitationPage />} />
        <Route path="/moment/:moment_id" element={<MomentPage />} />
        <Route path="/results/:session_id" element={<ResultsPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
